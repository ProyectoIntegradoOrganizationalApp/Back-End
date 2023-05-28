-- Your SQL goes here
CREATE TABLE achievement (
    id VARCHAR PRIMARY KEY NOT NULL,
    title VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    icon VARCHAR NOT NULL,
    category VARCHAR NOT NULL,
    states INT[] NOT NULL
);

INSERT INTO achievement VALUES
('1', 'Befriend', 'Make friends on the platform', 'friends.png', 'friend', '{1, 5, 20}'), -- Implementado
('2', 'Pioneer', 'Create your own projects', 'projects.png', 'project', '{1, 5, 25}'), -- Implementado
('3', 'Kanbanize', 'Utilize Kanban App in projects', 'kanban.png', 'app', '{1, 5}'), -- Implementado
('4', 'Timeliner', 'Utilize Timeline App in projects', 'timeline.png', 'app', '{1, 5}'), -- Implementado
('5', 'Project Party', 'Invite anyone to join your projects', 'invite.png', 'project', '{1, 20, 50}'), -- Implementado
('6', 'Column Creator', 'Create Kanban columns', 'columns.png', 'app', '{1, 15, 40}'), -- Implementado
('7', 'App Installer', 'Install complementary apps for projects', 'install.png', 'app', '{1, 25, 100}'), -- Implementado
('8', 'Kanban Commander', 'Assign tasks to project members in Kanban', 'assign.png', 'app', '{1, 5, 25}'), -- Implementado
('9', 'Timeline Captain', 'Assign tasks to project members in timeline', 'assign.png', 'app', '{1, 5, 25}'), -- Implementado
('10', 'Chat Starter', 'Initiate a chat conversation', 'chat.png', 'friend', '{1}'),
('11', 'Git Integration', 'Link Git branch with project tasks', 'git.png', 'friend', '{1, 5}'),
('12', 'Project Joiner', 'Join an existing project', 'join.png', 'project', '{1, 5, 20}') -- Implementado