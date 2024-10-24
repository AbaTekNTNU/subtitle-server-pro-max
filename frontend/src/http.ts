export async function post(url: string, data?: BodyInit): Promise<Response> {
  const response = await fetch(url, {
    method: "POST",
    credentials: "same-origin",
    headers: {
      "Content-Type": "application/json",
    },
    body: data,
  });

  if (!response.ok) {
    throw new Error("Failed to fetch");
  }

  return response;
}

export async function get<Type>(url: string): Promise<Type> {
  const response = await fetch(url);

  if (!response.ok) {
    throw new Error("Failed to fetch");
  }

  return await response.json();
}
