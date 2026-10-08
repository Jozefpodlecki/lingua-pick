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

INSERT INTO topic
(
    id,
    name,
    description
)
VALUES
    ('lifestyle', 'Lifestyle', 'Everyday habits, preferences, and ways of living; distinguish a habit from advice or a value judgment.'),
    ('politics', 'Politics', 'Public institutions, elections, political positions, and civic discussion with clearly attributed viewpoints.'),
    ('civic_participation', 'Civic participation', 'Community meetings, volunteering, voting procedures, and participation in public life.'),
    ('law_and_rights', 'Law and rights', 'Everyday legal terminology, rights, obligations, and descriptions of rules; language practice is not legal guidance.'),
    ('buildings_and_architecture', 'Buildings and architecture', 'Building types, visible structural elements, and descriptions of built spaces.'),
    ('construction_and_repairs', 'Construction and repairs', 'Common tools, materials, building work, and descriptions of household repairs.'),
    ('cooking_and_recipes', 'Cooking and recipes', 'Kitchen actions, ingredients, quantities, and the language of simple recipe steps.'),
    ('agriculture_and_gardening', 'Agriculture and gardening', 'Familiar crops, garden objects, seasonal tasks, and descriptions of growing plants.'),
    ('pets_and_animal_care', 'Pets and animal care', 'Everyday descriptions of pets, their needs, and routine care interactions.'),
    ('environment_and_sustainability', 'Environment and sustainability', 'Descriptions of waste, resources, conservation, and everyday environmental choices.'),
    ('geography_and_landscapes', 'Geography and landscapes', 'Familiar landforms, bodies of water, and descriptions of places and distances.'),
    ('history_and_heritage', 'History and heritage', 'Descriptions of past events, historical places, and local heritage using supplied context.'),
    ('arts_and_culture', 'Arts and culture', 'Art forms, exhibitions, cultural events, and descriptions of creative work.'),
    ('music_and_performance', 'Music and performance', 'Instruments, performances, musical preferences, and event participation.'),
    ('books_and_storytelling', 'Books and storytelling', 'Reading preferences, fictional events, characters, and simple story descriptions.'),
    ('news_and_media', 'News and media', 'Headlines, short reports, source attribution, and discussion of media content.'),
    ('science_and_discovery', 'Science and discovery', 'Everyday scientific descriptions, observations, and questions using explained terminology.'),
    ('digital_life', 'Digital life', 'Online accounts, messages, privacy settings, and common digital interactions.'),
    ('relationships_and_social_life', 'Relationships and social life', 'Friendship, social relationships, boundaries, and ordinary interpersonal interactions.'),
    ('celebrations_and_traditions', 'Celebrations and traditions', 'Invitations, celebrations, customs, and descriptions of local or family traditions.'),
    ('religion_and_beliefs', 'Religion and beliefs', 'Respectful descriptions of beliefs, practices, places, and personal viewpoints.'),
    ('emergencies_and_safety', 'Emergencies and safety', 'Reporting a problem, locating help, and understanding short safety notices.'),
    ('housing_and_renting', 'Housing and renting', 'Housing descriptions, rental enquiries, household arrangements, and repair requests.'),
    ('banking_and_finance', 'Banking and finance', 'Everyday accounts, payments, fees, and descriptions of financial transactions.'),
    ('business_and_trade', 'Business and trade', 'Orders, deliveries, customer enquiries, and straightforward business communication.'),
    ('manufacturing_and_tools', 'Manufacturing and tools', 'Common tools, manufactured objects, and simple descriptions of making things.'),
    ('materials_and_textures', 'Materials and textures', 'Common materials and tactile properties used to describe familiar objects.'),
    ('shapes_and_sizes', 'Shapes and sizes', 'Familiar shapes, dimensions, and relative size descriptions.'),
    ('diet_and_nutrition', 'Diet and nutrition', 'Food preferences, dietary needs, ingredients, and understanding ordinary labels.'),
    ('accessibility_and_inclusion', 'Accessibility and inclusion', 'Communicating access needs, accommodations, and inclusive everyday participation.');

INSERT INTO topic_stage
(
    topic_id,
    stage_code,
    generation_instructions
)
VALUES
    ('lifestyle', 'grammar', 'Use familiar routine verbs and isolate one introduced frequency or preference construction.'),
    ('politics', 'sentences', 'Use a short neutral statement about an institution or election; distinguish quoted opinion from fact and explain new terminology.'),
    ('civic_participation', 'sentences', 'Use one contextualized community invitation or procedural statement with familiar vocabulary; avoid assuming a particular political system.'),
    ('law_and_rights', 'sentences', 'Practice understanding a short stated rule or asking what a term means; provide the rule in the exercise rather than inventing applicable law.'),
    ('buildings_and_architecture', 'words', 'Match three to five familiar building or structural-element nouns to distinct English meanings; avoid technical design terminology.'),
    ('construction_and_repairs', 'words', 'Match familiar hand-tool or repair-material nouns from one small subtopic; do not give operational instructions.'),
    ('cooking_and_recipes', 'grammar', 'Use familiar ingredient and action words to practice one introduced quantity or instruction form; do not combine new grammar patterns.'),
    ('agriculture_and_gardening', 'words', 'Match familiar crop or garden-object nouns within one concrete set; avoid specialist agricultural terms.'),
    ('pets_and_animal_care', 'grammar', 'Use familiar animal vocabulary to practice one introduced need or possession construction; avoid veterinary advice.'),
    ('environment_and_sustainability', 'sentences', 'Use one short description of an environmental action; explain new terms and avoid turning practice into advocacy or unsupported claims.'),
    ('geography_and_landscapes', 'words', 'Match a small set of concrete landform nouns with distinct English meanings; do not require geography knowledge.'),
    ('history_and_heritage', 'sentences', 'Supply a brief factual context and practice one short past-event statement; grade language understanding rather than historical recall.'),
    ('arts_and_culture', 'sentences', 'Use a short event description or preference statement with introduced vocabulary; no specialist art knowledge is required.'),
    ('music_and_performance', 'sentences', 'Use a short concert or performance interaction; text exercises do not assess musical listening or pronunciation.'),
    ('books_and_storytelling', 'sentences', 'Supply a short original story or book description and ask about one explicit event before requiring inference.'),
    ('news_and_media', 'sentences', 'Use an original self-contained report; clearly label fictional scenarios and distinguish reported claims from verified facts.'),
    ('science_and_discovery', 'sentences', 'Provide any scientific information needed in the prompt; assess the language rather than unexplained specialist knowledge.'),
    ('digital_life', 'sentences', 'Use a short fictional message or settings request with familiar terminology; never request real account details.'),
    ('relationships_and_social_life', 'sentences', 'Provide an explicit relationship and social context for one short message; practice language without assuming personal circumstances.'),
    ('celebrations_and_traditions', 'sentences', 'State the relevant tradition in the exercise and practice an invitation or description; do not assume every speaker follows it.'),
    ('religion_and_beliefs', 'sentences', 'Use clearly attributed descriptions or personal statements; do not assume the learner or all speakers share a belief.'),
    ('emergencies_and_safety', 'sentences', 'Practice a short request for help or a supplied notice; assess communication without inventing emergency procedures.'),
    ('housing_and_renting', 'sentences', 'Use a short fictional enquiry with explicit needs and conditions; do not assume local rental rules.'),
    ('banking_and_finance', 'sentences', 'Use fictional amounts and a short payment or account enquiry; explain terms without offering financial advice.'),
    ('business_and_trade', 'sentences', 'Use one brief enquiry or order statement with explicit parties and familiar quantities; avoid unexplained commercial jargon.'),
    ('manufacturing_and_tools', 'words', 'Match familiar tool or manufactured-object nouns in one small set; do not test technical processes.'),
    ('materials_and_textures', 'words', 'Match concrete material nouns or distinct texture labels in separate sets; avoid testing adjective agreement.'),
    ('shapes_and_sizes', 'grammar', 'Use familiar object vocabulary to isolate one introduced size comparison or descriptive agreement pattern.'),
    ('diet_and_nutrition', 'sentences', 'Practice a short ingredient question or dietary preference with explicit context; do not prescribe diets or health outcomes.'),
    ('accessibility_and_inclusion', 'sentences', 'Use a respectful short request for a specified accommodation; do not infer disability or needs from personal traits.');
