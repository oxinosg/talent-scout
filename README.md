# Talent Scout

## Description
A simple command line tool to get the list of the 10 players who scored the most, and the 10 players who assisted the most.

The app creates a small cache of the data required to run for future executions

## Instructions to Run
### 1. Set up Sportradar Soccer API
- Signup for Sportradar API here, or use an existing account.
  https://console.sportradar.com/signup
- Follow the registration steps.
- Once logged in, select _Add Trials_, and pick Soccer API from the list.

### 2. Run using `API_KEY=xxx cargo run`

### 3. Available customization through the following environmental variables
| env key              | description                       | default                    | required |
|----------------------|-----------------------------------|----------------------------|----------|
| ACCOUNT_ACCESS_LEVEL | Sportradar account access level   | trial                      |          |
| API_BASE_URL         | API base url for sportradar's API | https://api.sportradar.com |          |
| API_KEY              | Sportradar account's API_KEY      |                            | true     |
| CACHE_LOCATION       | Location to store cache           | cache                      |          |
| COMPETITION_ID       | Competition ID to get stats for   | sr:competition:17          |          |

## Tests `cargo test`

## Future improvements
 - [ ] Give option to keep print more than 10 players
 - [ ] Keep track of dates the date was cached, and refresh every X minutes
 - [ ] Set up more extensive testing
 - [ ] Use clap for better CLI extensibility
 - [ ] Set up env_logger and better logging when debug mode is enabled
 - [ ] Better handling of env variables
 - [ ] Allow using CLI to change season / competition we are checking
