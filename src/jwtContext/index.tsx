import type { ReactNode } from "react";
import { useEffect, createContext, useState, useContext } from "react";
import { decodeJwt } from "jose";

export interface UserContextProps {
  setEmail:React.Dispatch<React.SetStateAction<string>>;
  email:string;
  jwt:string;
  setJwt:React.Dispatch<React.SetStateAction<string>>;
}

export const UserContext = createContext<UserContextProps>(
  {} as UserContextProps,
);
export const useUserContext = () => {
  const context = useContext(UserContext)
  return context
}

export function UserContextProvider({
  children
}: {
  children: ReactNode;
}): JSX.Element {
  const [email,setEmail] = useState("");
  const [jwt,setJwt] = useState("")

  useEffect(() => {
    const searchParams = new URLSearchParams(window.location.search);
    if (searchParams.get("jwt")) {
      const currentUrl = new URL(window.location.href);
      setJWTfunc(currentUrl.searchParams.get("jwt")!)
      currentUrl.searchParams.delete("jwt");
      history.pushState({}, "", currentUrl.href);
    }
    else if(localStorage.getItem("JWT")){
      setJwt(localStorage.getItem("JWT")!);
    }
  }, []);
  const setJWTfunc = async(jwt:string)=>{
    const decode = decodeJwt(jwt || "");
    console.log("aud:",decode);
    localStorage.setItem("JWT",jwt);
    setJwt(jwt);
  }
  
  return (
    <UserContext.Provider
      value={{
        setEmail,
        email,
        setJwt,
        jwt
      }}
    >
      {children}
    </UserContext.Provider>
  );
}
