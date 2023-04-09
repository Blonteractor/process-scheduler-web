# Base image
FROM envoyproxy/envoy:v1.25-latest

# Install gettext-base for envsubst
RUN apt-get update && apt-get install -y gettext-base

WORKDIR /usr/src/proxy
# Copy the configuration file into the container
COPY envoy.yaml envoy.yaml.template

# Expose the proxy server port
EXPOSE ${LISTEN_PORT} ${ADMIN_PORT}

# Start Envoy with the specified configuration
CMD envsubst < envoy.yaml.template > /tmp/envoy.yaml && /usr/local/bin/envoy -c /tmp/envoy.yaml