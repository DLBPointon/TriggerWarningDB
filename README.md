# Trigger Warning DB / TriggeredDB

Name is in the air to be honest.

### NOTE: AI - mainly Claude, has been used in some of this project, mainly in translating work from previous iterations of the project into rust as well as some of the more mudane tasks of extending html templates and generating css. At no point have I allowed it to go wild and just write tons of stuff, I have made sure I have gone through and reviewed it all. Any messy code you do find, you can rest assured that it's from a human unless expressely stated.

## About
A website and database dedicated to helping those with traumatic experiences enjoy media again, without the shock of a traumatic event. This project aims to add a degree of annotation to the current IMDB and TVDB projects, via the community.

This is a project, quite literally, 6 years in the making. Initial versions being React -> Angular -> Google Sheet (which still works very nicely!) -> Rocket.rs (now).

At no point will this project be monetised, donations are welcome and will only go towards the upkeep and maintenance of the project. If there are any donations, there will be a page to log the ins and out of the pot.

## What about doesthedogdie.com
- A login system stops you being able to view `triggers`, this data needs to be free and open.
- I understand needing to make money to keep it running, but I despise adverts on a site needed for this kind of purpose.
- Too many `triggers` that make it harder to really find what you're looking for.

Admittedly, it has gotten better since the last time I looked at it. But the above points still stand.

## So far
- Movies DB is at a v1
    - Graphical bar shows instances of triggers at X time in movie (skip the traumatic bits)
    - By default, collapsed blocks of traumatic events and descriptions (to stop reading about the event anyway)
    - Movie search is done with a number of filters

## ToDo:
- Ensure DB migrations will be set up and safe to use.
- Stats pages
    - One thing I want to do is have a stats page so you can see what producers always do this, what studio loves sprinkling in some suprising trauma e.t.c.
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
