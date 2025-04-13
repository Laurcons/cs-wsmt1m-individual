import { api } from "@/lib/api";
import { Todo } from "@/types/todo";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { toast } from "sonner";
import clsx from "clsx";

export function AddTodo() {
  const [newTodoTitle, setNewTodoTitle] = useState("");
  const queryClient = useQueryClient();

  const createMutation = useMutation({
    mutationFn: (title: string) => api.todos.create({ title }),
    onSuccess: () => {
      setNewTodoTitle("");
      toast.success("Todo created successfully");
    },
    onError: (error: Error) => {
      // Invalidate to refetch the correct state
      queryClient.invalidateQueries({ queryKey: ["todos"] });
      toast.error(error.message);
    },
    onMutate: async (title) => {
      // Cancel outgoing refetches
      await queryClient.cancelQueries({ queryKey: ["todos"] });

      // Snapshot the previous value
      const previousTodos = queryClient.getQueryData<{ todos: Todo[] }>([
        "todos",
      ]);

      // Create an optimistic todo
      const optimisticTodo: Todo = {
        id: Date.now(), // Temporary ID
        title,
        is_done: false,
      };

      // Optimistically update the cache
      queryClient.setQueryData<{ todos: Todo[] }>(["todos"], (old) => {
        if (!old) return { todos: [optimisticTodo] };
        return {
          todos: [...old.todos, optimisticTodo],
        };
      });

      return { previousTodos };
    },
    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: ["todos"] });
    },
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!newTodoTitle.trim()) return;
    createMutation.mutate(newTodoTitle.trim());
  };

  const isCreating = createMutation.isPending;

  return (
    <form onSubmit={handleSubmit} className="mb-8">
      <div className="flex gap-2">
        <input
          type="text"
          value={newTodoTitle}
          onChange={(e) => setNewTodoTitle(e.target.value)}
          placeholder={isCreating ? "Creating todo..." : "Add a new todo..."}
          maxLength={512}
          disabled={isCreating}
          className={clsx(
            "flex-1 px-4 py-2 border rounded-lg text-black focus:outline-none focus:ring-2 focus:ring-blue-500 transition-colors",
            isCreating && "cursor-wait bg-gray-50"
          )}
        />
        <button
          type="submit"
          disabled={isCreating}
          className={clsx(
            "px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all",
            isCreating ? "cursor-wait opacity-50" : "cursor-pointer"
          )}
        >
          {isCreating ? "Adding..." : "Add"}
        </button>
      </div>
    </form>
  );
}
