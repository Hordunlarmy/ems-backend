INSERT INTO account_types (id, name, description)
VALUES 
('abf0e840-e29b-41d4-a716-446655440000', 'Admin', 'Full access to all EMS modules'),
('abf0e840-e29b-41d4-a716-446655440001', 'Employee', 'Standard employee account'),
('abf0e840-e29b-41d4-a716-446655440002', 'Contractor', 'Limited access for contractors'),
('abf0e840-e29b-41d4-a716-446655440003', 'Visitor', 'Temporary access for visitors');

INSERT INTO permissions (id, name, description)
VALUES
('abf0e840-e29b-41d4-a716-446655440004', 'View Data', 'Permission to view data in the system'),
('abf0e840-e29b-41d4-a716-446655440005', 'Edit Data', 'Permission to edit data in the system'),
('abf0e840-e29b-41d4-a716-446655440006', 'Delete Data', 'Permission to delete data in the system'),
('abf0e840-e29b-41d4-a716-446655440007', 'Create Data', 'Permission to create new data in the system'),
('abf0e840-e29b-41d4-a716-446655440008', 'Manage Users', 'Permission to manage user accounts'),
('abf0e840-e29b-41d4-a716-446655440009', 'Manage Roles', 'Permission to manage user roles'),
('abf0e840-e29b-41d4-a716-446655440010', 'Manage Permissions', 'Permission to manage user permissions'),
('abf0e840-e29b-41d4-a716-446655440011', 'View Reports', 'Permission to view reports in the system'),
('abf0e840-e29b-41d4-a716-446655440012', 'Export Data', 'Permission to export data from the system');

INSERT INTO roles (id, name, description)
VALUES
('abf0e840-e29b-41d4-a716-446655440013', 'Super Admin', 'Full administrative access');

INSERT INTO role_permissions (id, role_id, permission_id) VALUES
('abf0e840-e29b-41d4-a716-446655440014', 'abf0e840-e29b-41d4-a716-446655440013', 'abf0e840-e29b-41d4-a716-446655440004');

INSERT INTO accounts (id, first_name, last_name, email, password, account_type_id, access_level, status)
VALUES
('f47ac10b-58cc-4372-a567-0e02b2c3d479', 'Alice', 'Admin', 'admin@ems.com', crypt('password', gen_salt('bf')), 'abf0e840-e29b-41d4-a716-446655440000', 10, 'active');

