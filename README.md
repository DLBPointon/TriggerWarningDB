# Trigger Warning

## About
A website and database dedicated to helping those with traumatic experiences enjoy media again, without the shock of a traumatic event. This project aims to add a degree of annotation to the current IMDB and TVDB projects, via the community.

This is a project, quite literally, 6 years in the making. Initial versions being React -> Angular -> Google Sheet (which still works very nicely!) -> Rocket.rs.

At no point will this project be monetised, donations are welcome and will only go towards the upkeep and maintenance of the project.

## ToDo:
- TV media
    - Adding support for TV media is the next major plan after the movie side is stabilised.
    - There will be extra work on this simply because of the extra layer of data needed to be kept.
- Points that need expert opinion
    - Event Categories
        - This should be a small enough list to cover a wide range of topics
            - One thing i'm keen on is keeping this concise
            - However, that should only be second to keeping the nuance of these terms.
        - Are what we have enough to capture an event type without losing the nuance of that event?
            - Is Birth Trauma too "generic"?
            - Should it be seperated into:
                - "Birth Event"
                - "Child Loss"
                - "Parent Loss"
                - "Medically Difficult Birth Event"
            - Likewise with "Sexual Violence"
                - It is a very broad term. 
                - Should it also be split out.
            - And then the interplay between various events.
                - The effect of CSA on Birth, should this be it's own topic.

## Tech Stack
- Rocket.rs
    - A rust web framework, simply it has made the most sense to me and this is the furthest I've got with the many iterations of this project that there has been.
- Diesel
    - Object Relational Mapper (ORM), this is what allows the Rocket app to talk to the Database.
- Sqlite3
    - Used simply because it was the default, I may change this out for postgresql in the future but for now sqlite3 works well.
- Rust
    - Rust is the language used to write the vast majority of TriggerWarning, bar some SQL. It's core tenants revolve around: performance, type safety, concurrency, and memory safety.
