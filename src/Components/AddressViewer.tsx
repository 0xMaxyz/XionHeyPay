
const AddressViewer = (props:{address:string}) => {
  console.log("Address:",props.address)
  return (
    <div>{(props.address!.length > 20)?props.address.slice(0,16)+"..."+props.address.slice(-4): props.address}</div>
  )
}

export default AddressViewer