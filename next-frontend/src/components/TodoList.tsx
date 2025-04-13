import { api } from "@/lib/api";
import { useQuery } from "@tanstack/react-query";
import { AddTodo } from "./AddTodo";
import { TodoItem } from "./TodoItem";
import { TodoSkeleton } from "./TodoSkeleton";

function EmptyState() {
  return (
    <div className="text-center py-12">
      <svg
        className="mx-auto h-12 w-12 text-gray-400"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
        aria-hidden="true"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={2}
          d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
        />
      </svg>
      <h3 className="mt-2 text-sm font-medium text-gray-900">No todos</h3>
      <p className="mt-1 text-sm text-gray-500">
        Get started by creating a new todo.
      </p>
    </div>
  );
}

export function TodoList() {
  const { data, isLoading, error } = useQuery({
    queryKey: ["todos"],
    queryFn: () => api.todos.list(),
  });

  if (isLoading) {
    return (
      <div className="max-w-2xl mx-auto p-4">
        <div className="mb-8 animate-pulse">
          <div className="flex gap-2">
            <div className="flex-1 h-10 bg-gray-200 rounded-lg" />
            <div className="w-20 h-10 bg-gray-200 rounded-lg" />
          </div>
        </div>
        <div className="space-y-2">
          {[...Array(3)].map((_, i) => (
            <TodoSkeleton key={i} />
          ))}
        </div>
      </div>
    );
  }

  if (error instanceof Error) {
    return <div className="text-red-500 text-center py-4">{error.message}</div>;
  }

  const todos = data?.todos ?? [];

  return (
    <div className="max-w-2xl mx-auto p-4">
      <AddTodo />
      {todos.length === 0 ? (
        <EmptyState />
      ) : (
        <div className="space-y-2">
          {todos.map((todo) => (
            <TodoItem key={todo.id} todo={todo} />
          ))}
        </div>
      )}
    </div>
  );
}
