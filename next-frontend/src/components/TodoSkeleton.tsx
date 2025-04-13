export function TodoSkeleton() {
  return (
    <div className="flex items-center gap-2 p-4 bg-white rounded-lg shadow animate-pulse">
      <div className="h-5 w-5 rounded bg-gray-200" />
      <div className="flex-1 h-4 bg-gray-200 rounded" />
      <div className="h-5 w-5 rounded bg-gray-200" />
    </div>
  );
}
