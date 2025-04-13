export interface Todo {
  id: number;
  title: string;
  is_done: boolean;
}

export interface CreateTodoInput {
  title: string;
}

export interface UpdateTodoInput {
  title?: string;
  is_done?: boolean;
}

export interface ListTodosQuery {
  is_done?: string;
}
