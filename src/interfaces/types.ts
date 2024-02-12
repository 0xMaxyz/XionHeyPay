export interface BalanceRow{
    token_address:string,
    symbol:string,
    amount:number,
    decimals:number,
    logo:string,
    price:number,
}
export interface ClaimRow{
    sender:string,
    metadata:string,
    symbol:string,
    token_address:string,
    amount:number,
    decimals:number,
    logo:string,
    price:number,
}