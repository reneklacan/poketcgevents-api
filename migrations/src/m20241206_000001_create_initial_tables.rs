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
                discord_id  TEXT NOT NULL UNIQUE,
                nickname    TEXT NOT NULL,
                avatar_url  TEXT,
                is_verified BOOLEAN,
                created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
            );

            -- GOOGLE USERS

            CREATE TABLE google_users (
                id                 SERIAL PRIMARY KEY,
                google_id          TEXT NOT NULL UNIQUE,
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
                timezone    TEXT NOT NULL,
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

            ALTER TABLE users
                ADD CONSTRAINT fk_users_discord_id
                FOREIGN KEY (discord_id) REFERENCES discord_users (discord_id);
            ALTER TABLE users
                ADD CONSTRAINT fk_users_google_id
                FOREIGN KEY (google_id) REFERENCES google_users (google_id);
            ALTER TABLE events
                ADD CONSTRAINT fk_events_organizer_id
                FOREIGN KEY (organizer_id) REFERENCES organizers (id);
            ALTER TABLE user_subscriptions
                ADD CONSTRAINT fk_user_subscriptions_user_id
                FOREIGN KEY (user_id) REFERENCES users (id);
            ALTER TABLE user_subscription_notifications
                ADD CONSTRAINT fk_user_subscription_notifications_user_subscription_id
                FOREIGN KEY (user_subscription_id) REFERENCES user_subscriptions (id);
            ALTER TABLE user_subscription_notifications
                ADD CONSTRAINT fk_user_subscription_notifications_event_id
                FOREIGN KEY (event_id) REFERENCES events (id);

            -- UNIQUE CONSTRAINTS

            ALTER TABLE discord_users
                ADD CONSTRAINT uk_discord_users_discord_id
                UNIQUE (discord_id);
            ALTER TABLE google_users
                ADD CONSTRAINT uk_google_users_google_id
                UNIQUE (google_id);
            ALTER TABLE events
                ADD CONSTRAINT uk_events_guid
                UNIQUE (guid);

            CREATE UNIQUE INDEX idx_user_subscription_notifications_subscription_id_event_id
                ON user_subscription_notifications (user_subscription_id, event_id);
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
            DROP TABLE IF EXISTS users;
            DROP TABLE IF EXISTS google_users;
            DROP TABLE IF EXISTS discord_users;
        "#,
        )
        .await?;

        Ok(())
    }
}
