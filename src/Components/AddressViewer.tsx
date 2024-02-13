import React from 'react'
const AddressViewer = (props:{address:string}) => {
  return (
    <div>{(props.address!.length > 20)?props.address.slice(0,8)+"..."+props.address.slice(-4): props.address}</div>
  )
}

export default AddressViewer