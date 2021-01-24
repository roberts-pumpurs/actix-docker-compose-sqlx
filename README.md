
DB setup

    docker exec -i actix-todo-dev-db mysql -uroot -ppassword < db/schema.sql

Start project PRODUCTION

    docker-compose -f docker-compose.yml -f docker-compose.prod.yml build api
    docker-compose -f docker-compose.yml -f docker-compose.prod.yml up

Start project DEVELOPMENT

    docker-compose -f docker-compose.yml -f docker-compose.dev.yml build api
    docker-compose -f docker-compose.yml -f docker-compose.dev.yml up
