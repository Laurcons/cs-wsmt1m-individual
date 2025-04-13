import { api } from "@/lib/api";
import { Todo } from "@/types/todo";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { toast } from "sonner";
import { TrashIcon } from "./icons/TrashIcon";
import clsx from "clsx";

interface TodoItemProps {
  todo: Todo;
}

export function TodoItem({ todo }: TodoItemProps) {
  const [editingTitle, setEditingTitle] = useState("");
  const [isEditing, setIsEditing] = useState(false);
  const queryClient = useQueryClient();

  const updateMutation = useMutation({
    mutationFn: ({ id, ...data }: Partial<Todo> & { id: number }) =>
      api.todos.update(id, data),
    onSuccess: () => {
      setIsEditing(false);
      toast.success("Todo updated successfully");
    },
    onError: (error: Error, variables) => {
      // Invalidate to refetch the correct state
      queryClient.invalidateQueries({ queryKey: ["todos"] });
      toast.error(error.message);
      setIsEditing(false);
    },
    onMutate: async ({ id, ...update }) => {
      // Cancel outgoing refetches
      await queryClient.cancelQueries({ queryKey: ["todos"] });

      // Snapshot the previous value
      const previousTodos = queryClient.getQueryData<{ todos: Todo[] }>([
        "todos",
      ]);

      // Optimistically update the cache
      queryClient.setQueryData<{ todos: Todo[] }>(["todos"], (old) => {
        if (!old) return { todos: [] };
        return {
          todos: old.todos.map((t) => (t.id === id ? { ...t, ...update } : t)),
        };
      });

      return { previousTodos };
    },
    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: ["todos"] });
    },
  });

  const deleteMutation = useMutation({
    mutationFn: (id: number) => api.todos.delete(id),
    onSuccess: () => {
      toast.success("Todo deleted successfully");
    },
    onError: (error: Error) => {
      // Invalidate to refetch the correct state
      queryClient.invalidateQueries({ queryKey: ["todos"] });
      toast.error("Failed to delete todo. Please try again.");
    },
    onMutate: async (deletedId) => {
      // Cancel outgoing refetches
      await queryClient.cancelQueries({ queryKey: ["todos"] });

      // Snapshot the previous value
      const previousTodos = queryClient.getQueryData<{ todos: Todo[] }>([
        "todos",
      ]);

      // Optimistically update the cache
      queryClient.setQueryData<{ todos: Todo[] }>(["todos"], (old) => {
        if (!old) return { todos: [] };
        return {
          todos: old.todos.filter((t) => t.id !== deletedId),
        };
      });

      return { previousTodos };
    },
    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: ["todos"] });
    },
  });

  const startEditing = () => {
    setIsEditing(true);
    setEditingTitle(todo.title);
  };

  const handleEditKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Escape") {
      setIsEditing(false);
    } else if (e.key === "Enter") {
      const newTitle = editingTitle.trim();
      if (newTitle && newTitle !== todo.title) {
        updateMutation.mutate({ id: todo.id, title: newTitle });
      } else {
        setIsEditing(false);
      }
    }
  };

  const isUpdating = updateMutation.isPending;
  const isDeleting = deleteMutation.isPending;

  return (
    <div
      className={clsx(
        "flex items-center gap-2 p-4 bg-white rounded-lg shadow transition-opacity",
        (isUpdating || isDeleting) && "opacity-60"
      )}
    >
      <input
        type="checkbox"
        checked={todo.is_done}
        disabled={isUpdating}
        onChange={() =>
          updateMutation.mutate({ id: todo.id, is_done: !todo.is_done })
        }
        className={clsx(
          "h-5 w-5 rounded border-gray-300 text-blue-500 focus:ring-blue-500 transition-colors",
          isUpdating && "cursor-wait"
        )}
      />
      {isEditing ? (
        <input
          type="text"
          value={editingTitle}
          onChange={(e) => setEditingTitle(e.target.value)}
          onKeyDown={handleEditKeyDown}
          onBlur={() => setIsEditing(false)}
          disabled={isUpdating}
          maxLength={512}
          autoFocus
          className={clsx(
            "flex-1 px-2 py-1 text-black bg-gray-50 border rounded focus:outline-none focus:ring-2 focus:ring-blue-500",
            isUpdating && "cursor-wait"
          )}
        />
      ) : (
        <span
          onClick={() => !todo.is_done && !isUpdating && startEditing()}
          className={clsx(
            "flex-1",
            todo.is_done
              ? "line-through text-gray-500"
              : "text-black cursor-pointer hover:text-blue-600",
            isUpdating && "cursor-wait"
          )}
        >
          {todo.title}
        </span>
      )}
      <button
        onClick={() => deleteMutation.mutate(todo.id)}
        disabled={isDeleting || isUpdating}
        className={clsx(
          "text-red-500 hover:text-red-700 focus:outline-none transition-colors",
          (isDeleting || isUpdating) && "cursor-wait opacity-50"
        )}
      >
        <TrashIcon />
      </button>
    </div>
  );
}
