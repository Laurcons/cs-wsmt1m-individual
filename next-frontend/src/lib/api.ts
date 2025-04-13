import {
  CreateTodoInput,
  ListTodosQuery,
  Todo,
  UpdateTodoInput,
} from "@/types/todo";

const API_BASE = "http://localhost:25565";

export class ApiError extends Error {
  constructor(public status: number, message: string) {
    super(message);
  }
}

async function handleResponse<T>(response: Response): Promise<T> {
  if (!response.ok) {
    const error = await response
      .json()
      .catch(() => ({ message: "An unknown error occurred" }));
    if (response.status === 422) {
      const keys = Object.keys(error);
      throw new ApiError(response.status, error[keys[0]][0].message);
    }
    throw new ApiError(response.status, error.message);
  }
  if (response.status === 204) {
    return {} as T;
  }
  return response.json();
}

export const api = {
  todos: {
    list: async (query?: ListTodosQuery): Promise<{ todos: Todo[] }> => {
      const queryString = query?.is_done ? `?is_done=${query.is_done}` : "";
      const response = await fetch(`${API_BASE}/todos${queryString}`);
      return handleResponse(response);
    },

    create: async (data: CreateTodoInput): Promise<void> => {
      const response = await fetch(`${API_BASE}/todos`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    update: async (id: number, data: UpdateTodoInput): Promise<void> => {
      const response = await fetch(`${API_BASE}/todos/${id}`, {
        method: "PATCH",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
      });
      return handleResponse(response);
    },

    delete: async (id: number): Promise<void> => {
      const response = await fetch(`${API_BASE}/todos/${id}`, {
        method: "DELETE",
      });
      return handleResponse(response);
    },
  },
};
