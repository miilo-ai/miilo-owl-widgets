"""Precompute provider-scoped links so compact clients avoid quadratic scans."""

def link(data):
    works = {(w['providerId'], w['id']): w for w in data['workItems']}
    actors = {(a['providerId'], a['id']): a for a in data['actors']}
    attempts = {}
    for work in works.values():
        work['_actorKeys'], work['_evidenceKeys'] = [], []
    for attempt in data['attempts']:
        provider = attempt['providerId']
        work = works.get((provider, attempt.get('taskId')))
        attempts[(provider, attempt['id'])] = work
        if work is not None:
            for actor_id in attempt.get('actorIds') or []:
                actor = actors.get((provider, actor_id))
                if actor is not None and actor['_key'] not in work['_actorKeys']:
                    work['_actorKeys'].append(actor['_key'])
    for evidence in data['evidence']:
        work = attempts.get((evidence['providerId'], evidence.get('attemptId')))
        if work is not None:
            work['_evidenceKeys'].append(evidence['_key'])
    for attention in data['attention']:
        subject = attention.get('subject') or {}
        work = works.get((attention['providerId'], subject.get('id'))) if subject.get('kind') in ('task', 'initiative') else None
        attention['_workKey'] = '' if work is None else work['_key']
        attention['_title'] = attention.get('message', '') if work is None else work['title']
    return data
