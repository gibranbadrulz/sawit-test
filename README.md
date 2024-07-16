# Sawit Test

SRE Technical Test from Sawit Pro, built with Rust as CLI tool. This tool is used to collect log data from source and ship into Elasticsearch Data Lake.

![Rust](https://img.shields.io/badge/Rust-%23000000.svg?style=flat&logo=rust&logoColor=white)
![ElasticSearch](https://img.shields.io/badge/ElasticSearch-005571?style=flat&logo=elasticsearch)
![Kibana](https://img.shields.io/badge/Kibana-A81C7D?style=flat&logo=kibana)


---

- [Architecture](#architecture)
- [Usage](#usage)
  - [Docker](#docker)
  - [— or](#-or)
- [Tests](#tests)
- [Rationale for Choosing thisInfrastructure](#rationale-for-choosing-this-infrastructure)
- [Elastic Stack](#elastic-stack)
  - [Elasticsearch Service](#elasticsearch-service)
  - [Kibana Service](#kibana-service)
  - [Deploy ELK Stack](#deploy-elk-stack)
  - [Deploy Kibana Dashboard](#deploy-kibana-dashboard)
  - [Dashboard Overview](#dashboard-overview)

---

## Architecture

Here's a simple diagram about the architecture of the tool which describes what are the main components and how they interact together.

![components](./assets/images/diagram.png)

## Usage

### Docker

```
$ docker run --rm \
  -e "ELASTIC_URL=http://127.0.0.1:9200" \
  -e "ELASTIC_USER=elastic" \
  -e "ELASTIC_PASSWORD=homestead" \
  -v ./sample.log:/tmp/sample.log \
  gibranbadrul/sawit-log --log-file /tmp/sample.log
```

### — or

Manual building executable from source code:

```
$ git clone https://github.com/gibranbadrulz/sawit-test.git
$ cd sawit-test
$ make release DEBUG=true
$ cp .env_example .env
$ ./target/release/sawit-log --log-file sample.log
```

Cleaning the project:

```
$ make clean
```

## Tests

![run](./assets/images/docker_run.png)

## Rationale for Choosing this Infrastructure

The chosen logging infrastructure is designed to be efficient, scalable, flexible, and reliable.

By leveraging asynchronous processing with `tokio` and `reqwest`, the system ensures non-blocking operations and can handle large volumes of log data efficiently. The use of buffered reading helps manage large log files without excessive memory usage.

`Elasticsearch` is a proven solution for log management, offering robust search capabilities and scalability, making it suitable for growing log data volumes.

Overall, this infrastructure balances performance, reliability, and flexibility, making it a robust choice for managing and analyzing log data.

## Elastic Stack

The `docker-compose.yml` file sets up Docker containers for Elasticsearch and Kibana, which are essential components of the Elastic Stack used for centralized logging and data visualization.

```yaml
version: "3.9"

services:
  elasticsearch:
    image: elasticsearch:7.17.12
    container_name: elasticsearch
    environment:
      - "node.name=elasticsearch"
      - "cluster.name=docker-cluster"
      - "ES_JAVA_OPTS=-Xms512m -Xmx512m"
      - "xpack.security.enabled=true"
      - "discovery.type=single-node"
      - "ELASTIC_PASSWORD=homestead"
    ports:
      - "9200:9200"
      - "9300:9300"
    networks:
      - es-net
    volumes:
      - esdata:/usr/share/elasticsearch/data

      ##
      ## kibana configurations here...
      ##

volumes:
  esdata:
    driver: local

networks:
  es-net:
    driver: bridge
```
### Elasticsearch Service

- **image:** Specifies the Elasticsearch Docker image and version (elasticsearch:7.17.12 in this case).
- **container_name:** Sets the name of the Docker container to elasticsearch.
- **environment:**
  - **node.name:** Specifies the name of the Elasticsearch node.
  - **cluster.name:** Sets the name of the Docker cluster.
  - **ES_JAVA_OPTS:** Sets JVM options for Elasticsearch heap size (-Xms512m -Xmx512m sets minimum and maximum heap size to 512MB).
  - **xpack.security.enabled:** Enables X-Pack security features.
  - **discovery.type:** Specifies the discovery type for Elasticsearch (single-node for standalone mode).
  - **ELASTIC_PASSWORD:** Sets the password for the elastic superuser.
- **ports:** Maps Elasticsearch ports:
  - **9200:9200:** Exposes Elasticsearch HTTP REST API.
  - **9300:9300:** Exposes Elasticsearch transport protocol.
- **networks:** Attaches the service to a custom Docker network named es-net.
- **volumes:** Mounts a Docker volume named esdata to persist Elasticsearch data.

```yaml
  kibana:
    image: kibana:7.17.12
    container_name: kibana
    environment:
      - "ELASTICSEARCH_URL=http://elasticsearch:9200"
      - "ELASTICSEARCH_HOSTS=http://elasticsearch:9200"
      - "ELASTICSEARCH_USERNAME=elastic"
      - "ELASTICSEARCH_PASSWORD=homestead"
    ports:
      - "5601:5601"
    networks:
      - es-net
    depends_on:
      - elasticsearch
```
### Kibana Service

- **image:** Specifies the Kibana Docker image and version (kibana:7.17.12 in this case).
- **container_name:** Sets the name of the Docker container to kibana.
- **environment:**
  - **ELASTICSEARCH_URL:** Specifies the URL of Elasticsearch.
  - **ELASTICSEARCH_HOSTS:** Specifies the hosts of Elasticsearch.
  - **ELASTICSEARCH_USERNAME:** Sets the username for Elasticsearch (elastic by default).
  - **ELASTICSEARCH_PASSWORD:** Sets the password for the elastic user.
- **ports:** Maps Kibana port 5601 to host port 5601.
- **networks:** Attaches the service to the same custom Docker network es-net as Elasticsearch.
- **depends_on:** Specifies that Kibana depends on Elasticsearch, ensuring Elasticsearch starts first.

### Deploy ELK Stack

To use this configuration:

```
docker compose up -d
```

Access Elasticsearch at http://localhost:9200 and Kibana at http://localhost:5601 in web browser.

### Deploy Kibana Dashboard

To deploy Kibana dashboards using saved objects stored in a [assets/dashboard.ndjson](./assets/dashboard.ndjson). Use `curl` or any HTTP client to send a POST request to import objects via Kibana Saved Objects API:

```
curl -X POST "http://elastic:homestead@localhost:5601/api/saved_objects/_import" \
  -H "kbn-xsrf: true" \
  --form file=@assets/dashboard.ndjson
```

### Dashboard Overview

![kibana](./assets/images/kibana.png)

The dashboard above displays relevant metrics such as error rates (e.g., count of status codes that are 400 or 500), average response times, total number of transactions and a lens visualization for detailed log analysis.
