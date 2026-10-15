.PHONY: assemble test check-owl
assemble:
	python3 tools/assemble.py

test:
	python3 -m unittest discover -s tests -p 'test_*.py' -v

check-owl:
	python3 tools/validate.py
