"use client";

import { TodoList } from "@/components/TodoList";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Toaster } from "sonner";

const queryClient = new QueryClient();

export default function Home() {
  return (
    <QueryClientProvider client={queryClient}>
      <main className="min-h-screen bg-gray-50 py-8">
        <div className="container mx-auto">
          <h1 className="text-3xl font-bold text-center mb-8 text-gray-800">
            Todo List
          </h1>
          <TodoList />
        </div>
      </main>
      <Toaster position="bottom-right" />
    </QueryClientProvider>
  );
}
