CREATE TABLE imu(id INTEGER NOT NULL, timestamp INTEGER NOT NULL PRIMARY KEY, 
    x_acceleration INTEGER NOT NULL, y_acceleration INTEGER NOT NULL, z_acceleration INTEGER NOT NULL, 
    x_gyro INTEGER NOT NULL, y_gyro INTEGER NOT NULL, z_gyro INTEGER NOT NULL);

CREATE TABLE wheel(id INTEGER NOT NULL, timestamp INTEGER NOT NULL PRIMARY KEY, 
    fl_wheel_speed INTEGER NOT NULL, fl_brake_temp REAL NOT NULL, fl_ambiant_temp REAL NOT NULL,
    fr_wheel_speed INTEGER NOT NULL, fr_brake_temp REAL NOT NULL, fr_ambiant_temp REAL NOT NULL,
    rl_wheel_speed INTEGER NOT NULL, rl_brake_temp REAL NOT NULL, rl_ambiant_temp REAL NOT NULL,
    rr_wheel_speed INTEGER NOT NULL, rr_brake_temp REAL NOT NULL, rr_ambiant_temp REAL NOT NULL);

CREATE TABLE datalog(id INTEGER NOT NULL, timestamp INTEGER NOT NULL PRIMARY KEY,
    drs INTEGER NOT NULL, steering_angle INTEGER NOT NULL, throttle_input REAL NOT NULL,
    front_brake_pressure REAL NOT NULL, rear_brake_pressure REAL NOT NULL,
    gps_lattitude REAL NOT NULL, gps_longitude REAL NOT NULL, 
    battery_voltage REAL NOT NULL, daq_current_draw REAL NOT NULL
);

CREATE TABLE ack(id INTEGER NOT NULL, timestamp INTEGER NOT NULL PRIMARY KEY);