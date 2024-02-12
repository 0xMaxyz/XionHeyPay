import type { ReactNode } from "react";
import { useEffect, createContext, useState, useContext } from "react";
import { decodeJwt } from "jose";
interface JWTObject{
  "https://stytch.com/session":{
    authentication_factors:[{
      email_factor:{
        email_address:string
      }
    }]
  }

}
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
      setJWTfunc(localStorage.getItem("JWT")!);
    }
  }, []);
  const setJWTfunc = async(jwt:string)=>{
    try {
      const decode = decodeJwt(jwt || "");
      console.log("aud:",decode);
      const decodeObject = decode as unknown as JWTObject;
      const email = decodeObject["https://stytch.com/session"].authentication_factors[0].email_factor.email_address;
      localStorage.setItem("JWT",jwt);
      console.log("Email:",email);
      console.log("JWT:",jwt);
      setJwt(jwt);
      setEmail(email);
    } catch (error) {
      console.log("Error Parsing JWT:",error)
    }

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
