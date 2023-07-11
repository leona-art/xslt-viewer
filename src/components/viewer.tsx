

export default function(props:{html:string,css:string}){
    return (
        <>
        <style>{props.css}</style>
        <div innerHTML={props.html}/>
        </>
    )
}