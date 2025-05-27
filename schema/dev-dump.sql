INSERT INTO accounts (id, first_name, last_name, email, password, account_type_id, access_level, status)
VALUES
('f47ac10b-58cc-4372-a567-0e02b2c3d480', 'Bob', 'Employee', 'employee@ems.com', crypt('password', gen_salt('bf')), 'abf0e840-e29b-41d4-a716-446655440001', 5, 'active'),
('f47ac10b-58cc-4372-a567-0e02b2c3d481', 'Charlie', 'Contractor', 'contractor@ems.com', crypt('password', gen_salt('bf')), 'abf0e840-e29b-41d4-a716-446655440002', 3, 'active'),
('f47ac10b-58cc-4372-a567-0e02b2c3d482', 'Diana', 'Visitor', 'visitor@ems.com', crypt('password', gen_salt('bf')), 'abf0e840-e29b-41d4-a716-446655440003', 1, 'active'),
('f47ac10b-58cc-4372-a567-0e02b2c3d483', 'Eve', 'Employee', 'employee1@ems.com', crypt('password', gen_salt('bf')), 'abf0e840-e29b-41d4-a716-446655440001', 3, 'inactive');

