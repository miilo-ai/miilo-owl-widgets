use crate::Result;

#[derive(Default)]
pub struct Answer {
    pending: String,
    depth: u8,
}
impl Answer {
    pub fn push(&mut self, text: &str, emit: &mut dyn FnMut(&str) -> Result<()>) -> Result<()> {
        const TAGS: [&str; 2] = ["<think>", "</think>"];
        self.pending.push_str(text);
        loop {
            let found = TAGS
                .iter()
                .enumerate()
                .filter_map(|(kind, tag)| {
                    self.pending.find(tag).map(|index| (index, kind, tag.len()))
                })
                .min_by_key(|value| value.0);
            if let Some((index, kind, length)) = found {
                if self.depth == 0 && index > 0 {
                    emit(&self.pending[..index])?;
                }
                if kind == 0 {
                    self.depth = self
                        .depth
                        .checked_add(1)
                        .filter(|depth| *depth <= 8)
                        .ok_or("Invalid reasoning frame")?;
                } else {
                    self.depth = self.depth.saturating_sub(1);
                }
                self.pending.drain(..index + length);
                continue;
            }
            let keep = TAGS
                .iter()
                .flat_map(|tag| {
                    (1..tag.len()).filter(|length| self.pending.ends_with(&tag[..*length]))
                })
                .max()
                .unwrap_or(0);
            let end = self.pending.len() - keep;
            if self.depth == 0 && end > 0 {
                emit(&self.pending[..end])?;
            }
            self.pending.drain(..end);
            return Ok(());
        }
    }
    pub fn finish(self, emit: &mut dyn FnMut(&str) -> Result<()>) -> Result<()> {
        if self.depth > 0 {
            return Err("Model reasoning did not finish".into());
        }
        if !self.pending.is_empty() {
            emit(&self.pending)?;
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn split_and_nested_reasoning_never_reaches_the_answer() {
        let mut filter = Answer::default();
        let mut output = String::new();
        for chunk in [
            "<th",
            "ink>private ",
            "<think>nested</think>",
            "reasoning</th",
            "ink>Final ",
            "answer.",
        ] {
            filter
                .push(chunk, &mut |text| {
                    output.push_str(text);
                    Ok(())
                })
                .unwrap();
        }
        filter
            .finish(&mut |text| {
                output.push_str(text);
                Ok(())
            })
            .unwrap();
        assert_eq!(output, "Final answer.");
    }
    #[test]
    fn unclosed_reasoning_fails_and_normal_unicode_is_preserved() {
        let mut filter = Answer::default();
        let mut output = String::new();
        filter
            .push("Hello 🙂", &mut |text| {
                output.push_str(text);
                Ok(())
            })
            .unwrap();
        assert_eq!(output, "Hello 🙂");
        filter
            .push("<think>private", &mut |text| {
                output.push_str(text);
                Ok(())
            })
            .unwrap();
        assert!(filter.finish(&mut |_| Ok(())).is_err());
        assert_eq!(output, "Hello 🙂");
    }
}
