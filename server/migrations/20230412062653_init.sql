CREATE TABLE IF NOT EXISTS users (
    id bigint NOT NULL,

    username varchar(255) NOT NULL,
    discrim smallint NOT NULL,
    phc varchar(255) NOT NULL,
    email varchar(255) UNIQUE NOT NULL,
    
    -- Nullable because the user may not have a profile image
    profile_img_id bigint DEFAULT NULL,
    accent_color char(6) DEFAULT NULL,
    pronouns varchar(255) DEFAULT NULL,
    bio varchar(255) DEFAULT NULL,
    
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS accessToken (
    token varchar(255) UNIQUE NOT NULL,
    user_id bigint NOT NULL,
    expires_at bigint NOT NULL,
    
    PRIMARY KEY (token),
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE IF NOT EXISTS guilds (
    id bigint NOT NULL,
    
    name varchar(255) NOT NULL,
    -- The owner of the guild. References users.id
    owner_id bigint NOT NULL,
    -- Timestamp -> cdn/guilds/<id>/<icon_id>.webp
    icon_id bigint DEFAULT NULL,

    PRIMARY KEY (id),
    FOREIGN KEY (owner_id) REFERENCES users (id)
);

CREATE TABLE IF NOT EXISTS roles (
    id bigint NOT NULL,
    guild_id bigint NOT NULL,
    
    name varchar(255) NOT NULL,
    color char(6) NOT NULL,
    permission_mask bigint NOT NULL,
    
    -- Linked list to sort this role's position in the hierarchy
    prev bigint DEFAULT NULL,
    next bigint DEFAULT NULL,
    
    PRIMARY KEY (id),
    FOREIGN KEY (guild_id) REFERENCES guilds (id),
    FOREIGN KEY (prev) REFERENCES roles (id),
    FOREIGN KEY (next) REFERENCES roles (id)
);

CREATE TABLE IF NOT EXISTS channels (
    id bigint NOT NULL,

    name varchar(255) NOT NULL,
    guild_id bigint NOT NULL,

    -- Linked list to sort this channel's position in the hierarchy
    prev bigint DEFAULT NULL,
    next bigint DEFAULT NULL,
    
    PRIMARY KEY (id),
    FOREIGN KEY (guild_id) REFERENCES guilds (id),
    FOREIGN KEY (prev) REFERENCES channels (id),
    FOREIGN KEY (next) REFERENCES channels (id)
);

CREATE TABLE IF NOT EXISTS messages (
    id bigint NOT NULL,
    channel_id bigint NOT NULL,
    author_id bigint NOT NULL,
    
    content varchar(16000) NOT NULL,
    
    updated_at bigint NOT NULL,
    
    PRIMARY KEY (id),
    FOREIGN KEY (channel_id) REFERENCES channels (id),
    FOREIGN KEY (author_id) REFERENCES users (id)
);

CREATE TABLE IF NOT EXISTS guild_members (
    id bigint NOT NULL,
    guild_id bigint NOT NULL,
    user_id bigint NOT NULL,

    -- Linked list to sort where the user placed this guild on the sidebar
    prev bigint DEFAULT NULL,
    next bigint DEFAULT NULL,
    
    PRIMARY KEY (id),
    FOREIGN KEY (guild_id) REFERENCES guilds (id),
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (prev) REFERENCES guild_members (id),
    FOREIGN KEY (next) REFERENCES guild_members (id)
);

CREATE TABLE IF NOT EXISTS guild_member_roles (
    id bigint NOT NULL,
    guild_member_id bigint NOT NULL,
    role_id bigint NOT NULL,
    
    PRIMARY KEY (id),
    FOREIGN KEY (guild_member_id) REFERENCES guild_members (id),
    FOREIGN KEY (role_id) REFERENCES roles (id)
);
