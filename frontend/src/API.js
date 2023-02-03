const API_URL = "https://es1.harryphillips.co.uk";

function createFormBody(obj) {
  let formBody = [];
  for (const property in obj) {
    const encodedKey = encodeURIComponent(property);
    const encodedValue = encodeURIComponent(obj[property]);
    formBody.push(encodedKey + "=" + encodedValue);
  }
  return formBody.join("&");
}

export function apiGet(endpoint) {
  return fetch(API_URL + endpoint, {
    credentials: 'include'
  });
}

export function apiPostForm(endpoint, formBody) {
  return fetch(API_URL + endpoint, {
    method: "POST",
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded;charset=UTF-8'
    },
    body: createFormBody(formBody),
    credentials: 'include'
  });
}

export function apiPostJson(endpoint, object) {
  return fetch(API_URL + endpoint, {
    method: "POST",
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(object),
    credentials: 'include'
  });
}
