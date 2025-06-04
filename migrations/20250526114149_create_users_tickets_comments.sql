-- Add migration script here
-- USERS TABLE
CREATE TABLE users (
    id          INT AUTO_INCREMENT PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    email       VARCHAR(100) NOT NULL UNIQUE,
    password    VARCHAR(255) NOT NULL,
    role        ENUM('admin', 'project_manager', 'qa', 'developer') NOT NULL DEFAULT 'developer',
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- TICKETS TABLE
CREATE TABLE tickets (
    id            INT AUTO_INCREMENT PRIMARY KEY,
    title         VARCHAR(200) NOT NULL,
    description   TEXT NOT NULL,
    status        ENUM('created', 'assigned', 'in_progress', 'resolved', 'tested', 'closed') NOT NULL DEFAULT 'created',
    priority      ENUM('low', 'medium', 'high') NOT NULL DEFAULT 'medium',
    created_by    INT NOT NULL,
    assigned_to   INT,
    created_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (assigned_to) REFERENCES users(id) ON DELETE SET NULL
);

-- COMMENTS TABLE
CREATE TABLE comments (
    id          INT AUTO_INCREMENT PRIMARY KEY,
    ticket_id   INT NOT NULL,
    user_id     INT NOT NULL,
    message     TEXT NOT NULL,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (ticket_id) REFERENCES tickets(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
