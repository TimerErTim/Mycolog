import * as apiProd from "../config/api.json"
import * as apiDev from "../config/api_dev.json"
import {dev} from "$app/environment";


export const api = {
    get backendProtocol() {
        if (dev) {
            return apiDev.backend_protocol
        } else {
            return apiProd.backend_protocol
        }
    },
    get backendHost() {
        if (dev) {
            return apiDev.backend_host
        } else {
            return apiProd.backend_host
        }
    },
    get backendPort() {
        if (dev) {
            return apiDev.backend_port
        } else {
            return apiProd.backend_port
        }
    },
    get backendEndpoint() {
        if (dev) {
            return apiDev.backend_endpoint
        } else {
            return apiProd.backend_endpoint
        }
    }
};

