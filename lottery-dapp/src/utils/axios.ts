import axios from "axios";

const base_url = import.meta.env.MODE === "DEVELOPEMENT" ? "" : "";
export const axiosInstance = axios.create({
  baseURL: base_url,
  withCredentials: true,
});
