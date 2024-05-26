import pyads

# Define the IP addresses and AMS addresses for the PC and PLC
pc_ip = "192.168.1.5"
pc_ams = "192.168.1.5.1.1"  # AMS address of the PC

plc_ip = "192.168.2.3"
plc_ams = "192.168.2.3.1.1"  # AMS address of the PLC

# Define credentials for accessing the PLC (if required)
plc_username = "Administrator"
plc_password = "1"

# Define the name for the route to be added
route_name = "plc"

# Open communication with the ADS server
pyads.open_port()

# Set the local AMS address for the PC
pyads.set_local_address(pc_ams)

# Add a route to connect to the PLC
pyads.add_route_to_plc(
    plc_ams,  # AMS address of the PLC
    pc_ip,  # IP address of the PC
    plc_ip,  # IP address of the PLC
    plc_username,  # Username for PLC access (if required)
    plc_password,  # Password for PLC access (if required)
    route_name,  # Name for the route
    pc_ams,  # AMS address of the PC (used for return communication)
)

# Close communication with the ADS server
pyads.close_port()
