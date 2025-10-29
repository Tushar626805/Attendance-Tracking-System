# Attendance Tracking System

A blockchain-based attendance management system built on the Stellar network using Soroban smart contracts. This decentralized solution provides transparent, immutable, and verifiable attendance records for events and workplaces.

## Table of Contents
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Smart Contract Overview](#smart-contract-overview)
- [Functions](#functions)
- [Data Structures](#data-structures)
- [Usage Guide](#usage-guide)
- [Future Scope](#future-scope)
- [Technical Requirements](#technical-requirements)
- [Deployment](#deployment)
- [Contributing](#contributing)

## Project Vision

To revolutionize attendance management by leveraging blockchain technology to create a transparent, tamper-proof, and efficient system for tracking attendance at events, workplaces, and educational institutions. Our vision is to eliminate attendance fraud, reduce administrative overhead, and provide verifiable proof of attendance that can be trusted by all stakeholders.

### Why Blockchain?

- **Immutability**: Once recorded, attendance data cannot be altered or deleted
- **Transparency**: All stakeholders can verify attendance records independently
- **Decentralization**: No single point of failure or control
- **Security**: Cryptographic signatures ensure authenticity
- **Trust**: Eliminates the need for intermediaries to verify attendance

## Key Features

### ✅ **Decentralized Attendance Marking**
- Users can mark their own attendance using cryptographic authentication
- Each attendance record is timestamped and linked to a specific event
- Prevents duplicate entries for the same user at the same event

### 📊 **Real-time Statistics**
- Track total number of attendees across all events
- Monitor total number of registered events
- Query individual attendance records instantly

### 🔐 **Secure Authentication**
- Utilizes Stellar's built-in address authentication
- Each user must cryptographically sign their attendance
- Admin-controlled event registration for added security

### 🌐 **Event Management**
- Admins can register new events with unique IDs
- Each event maintains its own attendance roster
- Scalable architecture supports unlimited events

### 📝 **Immutable Records**
- All attendance data is permanently stored on-chain
- Timestamps provide proof of when attendance was marked
- Records can be audited at any time

### 🚀 **Efficient Storage**
- Optimized data structures minimize storage costs
- TTL (Time To Live) management for long-term data persistence
- Lightweight contract design for fast execution

## Smart Contract Overview

The Attendance Tracking Smart Contract is built using the Soroban SDK and consists of four main functions:

1. **mark_attendance**: Records attendance for a user at a specific event
2. **get_attendance**: Retrieves attendance record for a user at an event
3. **get_stats**: Provides overall system statistics
4. **register_event**: Creates a new event (admin function)

## Functions

### `mark_attendance(user: Address, event_id: u64) -> bool`

**Description**: Marks attendance for a user at a specific event.

**Parameters**:
- `user`: The Stellar address of the attendee
- `event_id`: The unique identifier of the event

**Returns**: `true` if attendance was successfully marked, `false` if already marked

**Authentication**: Requires the user to sign the transaction

**Example**:
```rust
let success = contract.mark_attendance(&user_address, &1);
```

---

### `get_attendance(user: Address, event_id: u64) -> AttendanceRecord`

**Description**: Retrieves the attendance record for a specific user at a specific event.

**Parameters**:
- `user`: The Stellar address of the attendee
- `event_id`: The unique identifier of the event

**Returns**: `AttendanceRecord` structure containing:
- `user_address`: Address of the attendee
- `timestamp`: When attendance was marked
- `event_id`: Event identifier
- `is_present`: Boolean indicating if present

**Example**:
```rust
let record = contract.get_attendance(&user_address, &1);
```

---

### `get_stats() -> AttendanceStats`

**Description**: Returns overall system statistics.

**Returns**: `AttendanceStats` structure containing:
- `total_attendees`: Total number of attendance records
- `total_events`: Total number of registered events

**Example**:
```rust
let stats = contract.get_stats();
```

---

### `register_event(admin: Address) -> u64`

**Description**: Registers a new event in the system (admin only).

**Parameters**:
- `admin`: The Stellar address of the administrator

**Returns**: The unique `event_id` assigned to the new event

**Authentication**: Requires the admin to sign the transaction

**Example**:
```rust
let event_id = contract.register_event(&admin_address);
```

## Data Structures

### AttendanceRecord
```rust
pub struct AttendanceRecord {
    pub user_address: Address,  // Stellar address of attendee
    pub timestamp: u64,          // Unix timestamp of attendance
    pub event_id: u64,           // Unique event identifier
    pub is_present: bool,        // Attendance status
}
```

### AttendanceStats
```rust
pub struct AttendanceStats {
    pub total_attendees: u64,   // Total attendance records
    pub total_events: u64,      // Total registered events
}
```

## Usage Guide

### For Event Organizers/Admins:

1. **Register an Event**:
```bash
   soroban contract invoke \
     --id CONTRACT_ID \
     --source ADMIN_SECRET_KEY \
     -- register_event \
     --admin ADMIN_PUBLIC_KEY
```

2. **Check Statistics**:
```bash
   soroban contract invoke \
     --id CONTRACT_ID \
     -- get_stats
```

### For Attendees:

1. **Mark Your Attendance**:
```bash
   soroban contract invoke \
     --id CONTRACT_ID \
     --source USER_SECRET_KEY \
     -- mark_attendance \
     --user USER_PUBLIC_KEY \
     --event_id 1
```

2. **Verify Your Attendance**:
```bash
   soroban contract invoke \
     --id CONTRACT_ID \
     -- get_attendance \
     --user USER_PUBLIC_KEY \
     --event_id 1
```

## Future Scope

### Phase 1: Enhanced Features
- **QR Code Integration**: Generate unique QR codes for quick attendance marking
- **Geolocation Verification**: Ensure users are physically present at the event location
- **Time Window Restrictions**: Allow attendance only within specified time frames
- **Bulk Attendance Export**: Export attendance data in various formats (CSV, JSON, PDF)

### Phase 2: Advanced Functionality
- **Multi-Signature Approval**: Require multiple admins to approve critical operations
- **Role-Based Access Control**: Implement different permission levels (Admin, Manager, User)
- **Attendance Reports**: Generate automated reports with analytics and insights
- **Integration APIs**: REST APIs for integration with existing HR/Event management systems

### Phase 3: Ecosystem Expansion
- **NFT Certificates**: Issue NFT-based attendance certificates as proof of participation
- **Reward System**: Implement token rewards for consistent attendance
- **Mobile Application**: Native iOS and Android apps for easy attendance marking
- **Cross-Chain Support**: Expand to other blockchain networks

### Phase 4: Enterprise Features
- **AI-Powered Analytics**: Predict attendance patterns and identify trends
- **Biometric Integration**: Support for fingerprint and facial recognition
- **Compliance Tools**: Built-in features for regulatory compliance (GDPR, etc.)
- **Smart Notifications**: Automated reminders and attendance alerts
- **Delegation System**: Allow users to delegate attendance marking rights

### Additional Enhancements
- **Private Events**: Support for private/permissioned events
- **Recurring Events**: Handle weekly/monthly recurring events efficiently
- **Late Arrival Tracking**: Record exact arrival times for detailed analysis
- **Integration with DAOs**: Connect with decentralized autonomous organizations
- **Carbon Credit Tracking**: Link physical attendance to carbon offset programs

## Technical Requirements

### Prerequisites
- Rust 1.70 or higher
- Soroban CLI
- Stellar account with testnet/mainnet XLM

### Dependencies
```toml
[dependencies]
soroban-sdk = "20.0.0"
```

### Development Tools
- `soroban-cli`: For contract deployment and interaction
- `stellar-cli`: For account management
- `cargo`: Rust package manager

## Deployment

### 1. Build the Contract
```bash
cargo build --target wasm32-unknown-unknown --release
```

### 2. Optimize the WASM
```bash
soroban contract optimize \
  --wasm target/wasm32-unknown-unknown/release/attendance_contract.wasm
```

### 3. Deploy to Network
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/attendance_contract.wasm \
  --source DEPLOYER_SECRET_KEY \
  --network testnet
```

### 4. Initialize (if needed)
```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source ADMIN_SECRET_KEY \
  --network testnet \
  -- register_event \
  --admin ADMIN_PUBLIC_KEY
```

## Security Considerations

- Always verify user authentication before marking attendance
- Implement rate limiting to prevent spam attacks
- Regular security audits recommended before production use
- Keep admin keys secure and use hardware wallets when possible
- Monitor contract for unusual activity

## Testing

Run the included tests:
```bash
cargo test
```

For integration testing on testnet:
```bash
soroban contract invoke --id CONTRACT_ID --network testnet -- get_stats
```

## Contributing

We welcome contributions! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For questions and support:
- Open an issue on GitHub
- Join our Discord community
- Email: support@attendancechain.io

## Acknowledgments

- Built with [Soroban SDK](https://soroban.stellar.org/)
- Powered by [Stellar Network](https://stellar.org/)
- Inspired by the need for transparent attendance systems

---
## Contract Details
 

