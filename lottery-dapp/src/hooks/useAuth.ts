import { useQuery } from "@tanstack/react-query";
import { getAuthUser } from "../utils/api";

const useAuthUser = () => {
  const { data, isLoading } = useQuery({
    queryKey: ["authUser"],
    queryFn: getAuthUser,
    retry: false,
  });

  return { isLoading, authUser: data?.user || null };
};

export default useAuthUser;
