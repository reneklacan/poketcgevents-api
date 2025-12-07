use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            r#"
            -- USERS

            CREATE TABLE users (
                id          SERIAL PRIMARY KEY,
                username    TEXT,
                discord_id  TEXT UNIQUE,
                google_id   TEXT UNIQUE,
                created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
                updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
            );

            -- DISCORD USERS

            CREATE TABLE discord_users (
                id          SERIAL PRIMARY KEY,
                discord_id  TEXT NOT NULL,
                nickname    TEXT NOT NULL,
                avatar_url  TEXT,
                is_verified BOOLEAN,
                created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
            );

            -- GOOGLE USERS

            CREATE TABLE google_users (
                id                 SERIAL PRIMARY KEY,
                google_id          TEXT NOT NULL,
                email              TEXT NOT NULL,
                email_verified     BOOLEAN,
                first_name         TEXT,
                last_name          TEXT,
                profile_image_url  TEXT,
                created_at         TIMESTAMPTZ NOT NULL DEFAULT now()
            );

            -- ORGANIZERS

            CREATE TABLE organizers (
                id          SERIAL PRIMARY KEY,
                name        TEXT NOT NULL,
                address     TEXT NOT NULL,
                city        TEXT NOT NULL,
                area        TEXT NOT NULL,
                country     TEXT NOT NULL,
                latitude    FLOAT NOT NULL,
                longitude   FLOAT NOT NULL,
                created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
                updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
            );

            -- EVENTS

            CREATE TABLE events (
                id                  SERIAL PRIMARY KEY,
                organizer_id        INTEGER NOT NULL,
                kind                TEXT NOT NULL,
                name                TEXT NOT NULL,
                pokemon_event_slug  TEXT NOT NULL,
                guid                UUID NOT NULL,
                league              INTEGER,
                happening_at        TIMESTAMPTZ NOT NULL,
                created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
                updated_at          TIMESTAMPTZ NOT NULL DEFAULT now()
            );

            -- USER SUBSCRIPTIONS

            CREATE TABLE user_subscriptions (
                id              SERIAL PRIMARY KEY,
                user_id         INTEGER NOT NULL,
                destination     JSONB NOT NULL,
                search_filters  JSONB NOT NULL,
                notify_before   INTEGER NOT NULL,
                created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
                updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
            );

            -- USER SUBSCRIPTION NOTIFICATIONS

            CREATE TABLE user_subscription_notifications (
                id                   SERIAL PRIMARY KEY,
                user_subscription_id INTEGER NOT NULL,
                event_id             INTEGER NOT NULL,
                created_at           TIMESTAMPTZ NOT NULL DEFAULT now()
            );

            -- FOREIGN KEYS

            ALTER TABLE discord_users
                ADD CONSTRAINT fk_discord_users_users
                FOREIGN KEY (discord_id) REFERENCES users (discord_id);

            ALTER TABLE google_users
                ADD CONSTRAINT fk_google_users_users
                FOREIGN KEY (google_id) REFERENCES users (google_id);

            ALTER TABLE events
                ADD CONSTRAINT fk_events_organizers
                FOREIGN KEY (organizer_id) REFERENCES organizers (id);

            ALTER TABLE events
                ADD CONSTRAINT fk_events_guid
                UNIQUE (guid);

            ALTER TABLE user_subscriptions
                ADD CONSTRAINT fk_user_subscriptions_users
                FOREIGN KEY (user_id) REFERENCES users (id);

            ALTER TABLE user_subscription_notifications
                ADD CONSTRAINT fk_user_subscription_notifications_user_subscriptions
                FOREIGN KEY (user_subscription_id) REFERENCES user_subscriptions (id);

            ALTER TABLE user_subscription_notifications
                ADD CONSTRAINT fk_user_subscription_notifications_events
                FOREIGN KEY (event_id) REFERENCES events (id);
        "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            r#"
            DROP TABLE IF EXISTS user_subscription_notifications;
            DROP TABLE IF EXISTS user_subscriptions;
            DROP TABLE IF EXISTS events;
            DROP TABLE IF EXISTS organizers;
            DROP TABLE IF EXISTS google_users;
            DROP TABLE IF EXISTS discord_users;
            DROP TABLE IF EXISTS users;
        "#,
        )
        .await?;

        Ok(())
    }
}
