export const getAuthUser = async () => {
  try {
    const res = await axiosInstance.get("/api/auth/me", {
      withCredentials: true,
    });

    return res.data;
  } catch (error) {
    console.log("error in getAuthUser: ", error);
    return null;
  }
};
