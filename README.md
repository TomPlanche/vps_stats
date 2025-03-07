# Website Stats

Website Stats is a lightweight, privacy-friendly web analytics tool built with Rust. It provides an alternative to third-party analytics services by allowing you to capture and analyze visitor data on your own server.

## Features

- **Privacy-Focused**: Self-hosted and controlled, no data sent to third parties
- **Geographic Tracking**: Visualize visitor locations on a map
- **Browser & OS Statistics**: Track which browsers and operating systems your visitors use
- **Traffic Sources**: See where your visitors are coming from
- **Time-Based Analytics**: View hourly, daily, and weekly visitor patterns
- **Easy Integration**: Simple JavaScript snippet for your website
- **API Access**: RESTful API for querying analytics data

## Architecture & Diagrams

### Database Schema

![Database Schema](assets/database_schema.svg)

The database schema diagram illustrates the data model with three core tables:
- `city` - Stores geographic location data
- `collector` - Represents a visitor session with browser and OS information
- `event` - Tracks individual events like page views and clicks

### System Architecture

![System Architecture](assets/system_architecture.svg)

This diagram shows the overall components of the Website Stats system and how they interact, from the client-side script to the server-side API and database.

### Data Flow

![Data Flow](assets/data_flow.svg)

The data flow diagram traces how visitor data moves through the system, from initial collection to storage and analysis.

### Sequence Diagram

![Sequence Diagram](assets/sequence_diagram.svg)

This sequence diagram shows the temporal flow of operations when tracking a visitor, from initial page load to user interactions and page exit.

## Installation

### Prerequisites

- Rust and Cargo (1.76.0 or later)
- SQLite
- An IPinfo API token (for IP geolocation)

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/website_stats.git
   cd website_stats
   ```

2. Copy the example environment file and configure it:
   ```bash
   cp .env.exemple .env
   ```
   Edit `.env` to set your configuration values, including your IPinfo token.

3. Create the data directory:
   ```bash
   mkdir -p data
   ```

4. Run database migrations:
   ```bash
   cargo install diesel_cli --no-default-features --features sqlite
   diesel migration run
   ```

5. Build and run the application:
   ```bash
   cargo run
   ```

   The service will start on port 5775 by default.

## Configuration

The following environment variables can be configured in the `.env` file:

- `APP_URL`: The base URL of your application (default: `http://127.0.0.1:5775`)
- `SERVICE_PORT`: The port to run the service on (default: `5775`)
- `CORS_DOMAINS`: Comma-separated list of domains allowed to access the API
- `IPINFO_TOKEN`: Your IPinfo API token for geolocation
- `DEV`: Set to "true" for development mode, "false" for production

## Usage

### Adding the Tracking Code

Add the following script tag to the `<head>` of your website:

```html
<script async src="https://your-analytics-domain.com/stats.js"></script>
```

Replace `your-analytics-domain.com` with the domain where your Website Stats instance is running.

### Tracking Events

The tracking script automatically records page views. To track custom events:

```javascript
// Track a custom event
window.stats_collect("signup");

// Track a custom event with a specific URL
window.stats_collect("download", "https://example.com/files/document.pdf");
```

### Viewing Analytics

Analytics data is available through the REST API endpoints:

- `/summary/events`: Get overall event counts
- `/summary/five_minutes`: Get minute-by-minute data
- `/summary/hourly`: Get hourly data
- `/summary/browsers`: Get browser statistics
- `/summary/os_browsers`: Get OS and browser combinations
- `/summary/referrers`: Get referrer statistics
- `/session/map`: Get visitor geographic data

## API Documentation

### City Endpoints

- `POST /city`: Create a new city record
- `GET /city`: Get a list of cities

### Event Endpoints

- `POST /event`: Record a new event
- `GET /event`: Get a list of events

### Session Endpoints

- `GET /session`: Get recent visitor sessions
- `GET /session/map`: Get visitor map data

### Summary Endpoints

- `GET /summary/five_minutes`: Get minute-by-minute event summary
- `GET /summary/events`: Get event counts by time period
- `GET /summary/hourly`: Get hourly event summary
- `GET /summary/browsers`: Get browser statistics
- `GET /summary/os_browsers`: Get OS and browser statistics
- `GET /summary/referrers`: Get referrer statistics
- `GET /summary/percentages`: Get percentage changes in traffic

## Security Considerations

- This service collects visitor data including IP addresses and user agents. Ensure you comply with relevant privacy regulations like GDPR or CCPA.
- Consider placing the service behind a reverse proxy (like Nginx) with HTTPS for secure data transmission.
- By default, the service allows connections only from specified domains in the CORS_DOMAINS setting.
- No personally identifiable information is stored beyond IP addresses, which are used for geolocation.

## Development

### Running in Development Mode

Set `DEV=true` in your `.env` file to enable development mode, which:
- Uses a fake IP address for geolocation
- Provides additional debugging information

### Building for Production

```bash
cargo build --release
```

The compiled binary will be in `target/release/website_stats`.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgements

Based on [Udara's Stats project](https://github.com/UdaraJay/Stats).
