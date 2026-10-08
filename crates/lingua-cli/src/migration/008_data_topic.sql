INSERT INTO topic
(
    id,
    name,
    description
)
VALUES
    ('home_and_rooms', 'Home and rooms', 'Familiar rooms and household spaces; distinguish a room from an object inside it.'),
    ('clothing', 'Clothing', 'Common garments and accessories with clear everyday meanings.'),
    ('body_parts', 'Body parts', 'Common external body parts; begin with concrete nouns rather than medical terminology.'),
    ('colours', 'Colours', 'Common colour labels; separate recognition of a label from agreement or idiomatic uses.'),
    ('numbers_and_quantities', 'Numbers and quantities', 'Small cardinal numbers and common quantities before arithmetic, large numbers, or classifiers.'),
    ('nature_and_weather', 'Nature and weather', 'Familiar natural objects and weather conditions; distinguish a noun from a weather expression.'),
    ('transport', 'Transport', 'Common vehicles and travel objects appropriate to the selected region.'),
    ('school_and_learning', 'School and learning', 'Familiar classroom objects and learning activities without specialist academic vocabulary.'),
    ('work_and_jobs', 'Work and jobs', 'Common occupations, workplaces, and routine work actions; avoid unexplained professional jargon.'),
    ('hobbies_and_sports', 'Hobbies and sports', 'Frequent leisure activities, games, and sports; respect regional names and usage.'),
    ('shopping_and_money', 'Shopping and money', 'Everyday goods, prices, payment, and simple purchasing interactions.'),
    ('time_and_calendar', 'Time and calendar', 'Common time units, days, and routine scheduling; introduce one time convention at a time.'),
    ('greetings_and_introductions', 'Greetings and introductions', 'Greetings, names, and introductions with explicit social context and appropriate register.'),
    ('feelings_and_needs', 'Feelings and needs', 'Common feelings and immediate needs; provide enough context to distinguish related meanings.'),
    ('health_and_care', 'Health and care', 'Everyday descriptions of wellbeing and simple care interactions, without diagnostic or treatment advice.'),
    ('directions_and_location', 'Directions and location', 'Simple spatial relationships and directions with a clear reference point.'),
    ('travel_and_accommodation', 'Travel and accommodation', 'Short everyday interactions involving journeys, tickets, and accommodation.'),
    ('meals_and_restaurants', 'Meals and restaurants', 'Ordering, preferences, and simple meal descriptions using familiar food vocabulary.'),
    ('daily_routines', 'Daily routines', 'Short descriptions of regular activities and their sequence using familiar verbs.'),
    ('describing_people', 'Describing people', 'Neutral descriptions of people using familiar adjectives and introduced agreement patterns.'),
    ('describing_objects', 'Describing objects', 'Simple properties of familiar objects; isolate the targeted adjective or comparison.'),
    ('communication_and_technology', 'Communication and technology', 'Common devices and everyday communication actions without technical jargon.'),
    ('social_plans', 'Social plans', 'Simple invitations, availability, and shared plans with explicit time and register.'),
    ('public_services', 'Public services', 'Everyday interactions at common services using short, clearly contextualized requests.');

INSERT INTO topic_stage
(
    topic_id,
    stage_code,
    generation_instructions
)
VALUES
    ('home_and_rooms', 'words', 'Match three to five familiar room names to distinct English meanings; avoid sentences and spatial grammar.'),
    ('clothing', 'words', 'Match common garment nouns; avoid items with overlapping or regionally ambiguous labels.'),
    ('body_parts', 'words', 'Match common external body-part nouns with unambiguous English labels.'),
    ('colours', 'words', 'Match basic colour labels only; do not test adjective agreement or figurative meanings.'),
    ('numbers_and_quantities', 'words', 'Match a small set of cardinal-number words to English number words; avoid arithmetic and grammatical classifiers.'),
    ('nature_and_weather', 'words', 'Match concrete nature nouns or distinct weather labels; keep each set within one subtopic.'),
    ('transport', 'words', 'Match common vehicle nouns with meanings suitable for the exact target variety.'),
    ('school_and_learning', 'words', 'Match familiar classroom-object nouns; avoid abstract educational concepts.'),
    ('work_and_jobs', 'words', 'Match a few common occupation nouns; explain distinctions through clear English meanings.'),
    ('hobbies_and_sports', 'words', 'Match common sport or hobby labels without introducing activity sentences.'),
    ('shopping_and_money', 'grammar', 'Use familiar goods and one introduced quantity or grammatical relationship; do not assume knowledge of local currency conventions.'),
    ('time_and_calendar', 'grammar', 'Isolate one familiar time expression or verb form; make the time reference explicit.'),
    ('greetings_and_introductions', 'sentences', 'Use short introductions and greetings with explicit context; explain new expressions before assessing them.'),
    ('feelings_and_needs', 'grammar', 'Use familiar words to isolate one introduced expression of a feeling or need.'),
    ('health_and_care', 'sentences', 'Use short everyday requests with explicit context and familiar vocabulary; avoid medical advice.'),
    ('directions_and_location', 'grammar', 'Isolate one introduced spatial relationship with a clear referent and familiar objects.'),
    ('travel_and_accommodation', 'sentences', 'Use one short, contextualized travel interaction with familiar words and introduced grammar.'),
    ('meals_and_restaurants', 'sentences', 'Use a short request or meal description; clarify register and region-specific food terms.'),
    ('daily_routines', 'grammar', 'Practice one verb form using familiar daily actions; do not combine several new tenses.'),
    ('describing_people', 'grammar', 'Target one introduced agreement or adjective pattern using neutral descriptions.'),
    ('describing_objects', 'grammar', 'Target one introduced property, agreement, or comparison using familiar object nouns.'),
    ('communication_and_technology', 'sentences', 'Use a short everyday message or request with familiar device vocabulary.'),
    ('social_plans', 'sentences', 'Use short invitations or scheduling statements with explicit time and social context.'),
    ('public_services', 'sentences', 'Use one short service request; provide enough context to select a natural, appropriate form.');
