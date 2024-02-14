
export const HPCAddress =
  "xion1rd9a9rdw6gued7hf079a7gngyk0frnlqcc94y8xnhnkydsjzzzvsmxk5fk";
export const HaypayAddress = 
"xion1wxp3q98za6w9a2g5t2hvrdlh5lvasv0f3p085m0rquk9u4c8xeeqvyrnx6";
export interface token{
  token_address:string,
  logo:string,
  symbol:string,
  price:number
}
export const ValidCoins:token[]=[
  {
    token_address:"xion1rd9a9rdw6gued7hf079a7gngyk0frnlqcc94y8xnhnkydsjzzzvsmxk5fk",
    logo:"/HeyPay/USDT.png",
    symbol:"USDT",
    price:1
  },
  {
    token_address:"xion1098gd62n5glzqual82v8x7mandchzg5qktky3cdljpnhxcsdn78sy95dlu",
    logo:"/HeyPay/Xion.png",
    symbol:"Xion",
    price:20.1
  },
  {
    token_address:"xion1jfzcfyvznelca47utxwqt5nmc8c8yr9jcng3v8fz0jpert9kmtsqq23aml",
    logo:"/HeyPay/BTC.png",
    symbol:"BTC",
    price:50000
  }
]
function tm(){
  let TokenMaps = new Map<string,token>();
  for(let i=0;i<ValidCoins.length;i++){
    TokenMaps.set(ValidCoins[i].token_address,ValidCoins[i]);
  }
  return TokenMaps;
}

export const TokenMaps = tm();
console.log("Map", TokenMaps);
