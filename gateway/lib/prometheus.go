package lib

import (
	"fmt"
	"net/http"

	"github.com/prometheus/client_golang/prometheus/promhttp"
	"github.com/rs/zerolog/log"
)

// CreatePrometheus creates a webserver instance that returns the metrics of the
// current program reported using promauto.
func CreatePrometheus(port int) {
	http.Handle("/metrics", promhttp.Handler())
	err := http.ListenAndServe(fmt.Sprintf(":%d", port), nil)

	if err != nil {
		log.Err(err).Msgf("failed to start the prometheus reporting on the port :%d", port)
	}
}
