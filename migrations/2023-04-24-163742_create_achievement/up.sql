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
('1', 'Befriend', 'Make friends on the platform', 'https://www.svgrepo.com/show/513065/wifi-1012.svg', 'friend', '{1, 5, 20}'), -- Implementado
('2', 'Pioneer', 'Create your own projects', 'https://www.svgrepo.com/show/512254/flag-bug-report-1493.svg', 'project', '{1, 5, 25}'), -- Implementado
('3', 'Kanbanize', 'Utilize Kanban App in projects', 'https://www.svgrepo.com/show/512604/object-placement-65.svg', 'app', '{1, 5}'), -- Implementado
('4', 'Timeliner', 'Utilize Timeline App in projects', 'https://www.svgrepo.com/show/512922/stats-1369.svg', 'app', '{1, 5}'), -- Implementado
('5', 'Project Party', 'Invite anyone to join your projects', 'https://www.svgrepo.com/show/512401/key-677.svg', 'project', '{1, 20, 50}'), -- Implementado
('6', 'Column Creator', 'Create Kanban columns', 'https://www.svgrepo.com/show/511552/bookmark-plus-fill-1237.svg', 'app', '{1, 15, 40}'), -- Implementado
('7', 'App Installer', 'Install complementary apps for projects', 'https://www.svgrepo.com/show/512290/folder-plus-1782.svg', 'app', '{1, 25, 100}'), -- Implementado
('8', 'Kanban Commander', 'Assign tasks to project members in Kanban', 'https://www.svgrepo.com/show/512991/tie-763.svg', 'app', '{1, 5, 25}'), -- Implementado
('9', 'Timeline Captain', 'Assign tasks to project members in timeline', 'https://www.svgrepo.com/show/512992/tie-765.svg', 'app', '{1, 5, 25}'), -- Implementado
('10', 'Chat Starter', 'Initiate a chat conversation', 'https://www.svgrepo.com/show/512468/message-1579.svg', 'friend', '{1}'),
('11', 'Git Integration', 'Link Git branch with project tasks', 'https://www.svgrepo.com/show/512317/github-142.svg', 'friend', '{1, 5}'),
('12', 'Project Joiner', 'Join an existing project', 'https://www.svgrepo.com/show/511307/airplane-mode-1406.svg', 'project', '{1, 5, 20}') -- Implementado