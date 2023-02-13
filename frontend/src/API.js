const API_URL = "https://es1.harryphillips.co.uk/api";

function createFormBody(obj) {
  let formBody = [];
  for (const property in obj) {
    const encodedKey = encodeURIComponent(property);
    const encodedValue = encodeURIComponent(obj[property]);
    formBody.push(encodedKey + "=" + encodedValue);
  }
  return formBody.join("&");
}

async function handleResponse(response, dontHandleError) {
  if (response.ok || dontHandleError) return response;
  let message = await response.text();
  if (message !== "") {
    window.alert("Request failed (" + response.status + " " + response.statusText + "):" + message);
  } else {
    window.alert("Request failed (" + response.status + " " + response.statusText + ")");
  }
  window.location = "/app";
}

export function apiGet(endpoint, dontHandleError = false) {
  return fetch(API_URL + endpoint, {
    credentials: "include",
  }).then((r) => handleResponse(r, dontHandleError));
}

export function apiPost(endpoint, dontHandleError = false) {
  return fetch(API_URL + endpoint, {
    method: "POST",
    credentials: "include",
  }).then((r) => handleResponse(r, dontHandleError));
}

export function apiPostForm(endpoint, formBody, dontHandleError = false) {
  return fetch(API_URL + endpoint, {
    method: "POST",
    headers: {
      "Content-Type": "application/x-www-form-urlencoded;charset=UTF-8",
    },
    body: createFormBody(formBody),
    credentials: "include",
  }).then((r) => handleResponse(r, dontHandleError));
}

export function apiPostJson(endpoint, object, dontHandleError = false) {
  return fetch(API_URL + endpoint, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(object),
    credentials: "include",
  }).then((r) => handleResponse(r, dontHandleError));
}
