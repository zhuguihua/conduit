package srv

import (
	"net/http"

	"github.com/julienschmidt/httprouter"
	"github.com/runconduit/conduit/controller/api/public"
	pb "github.com/runconduit/conduit/controller/gen/public"
	log "github.com/sirupsen/logrus"
)

type (
	renderTemplate func(http.ResponseWriter, string, string, interface{}) error
	serveFile      func(http.ResponseWriter, string, string, interface{}) error

	handler struct {
		render    renderTemplate
		serveFile serveFile
		apiClient public.ConduitApiClient
		uuid      string
	}
)

func (h *handler) handleIndex(w http.ResponseWriter, req *http.Request, p httprouter.Params) {
	params := appParams{UUID: h.uuid}

	version, err := h.apiClient.Version(req.Context(), &pb.Empty{}) // TODO: remove and call /api/version from web app
	if err != nil {
		params.Error = true
		params.ErrorMessage = err.Error()
		log.Error(err.Error())
	} else {
		params.Data = version
	}

	err = h.render(w, "app.tmpl.html", "base", params)

	if err != nil {
		log.Error(err.Error())
		w.WriteHeader(http.StatusInternalServerError)
	}
}
