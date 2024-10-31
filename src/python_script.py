import inflect

p = inflect.engine()

def get_plural_of_noun(noun: str) -> str:
    return p.plural_noun(noun)

def get_plural_of_verb(verb: str) -> str:
    return p.plural_verb(verb)
