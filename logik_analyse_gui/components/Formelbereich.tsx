import Formel from '../components/Formel'
import Paper from '@mui/material/Paper';
import Stack from '@mui/material/Stack';
import Button from '@mui/material/Button';
import { styled } from '@mui/material/styles';
import Werkzeugkasten from '../components/Werkzeugkasten'
import { Children, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import IconButton from '@mui/material/IconButton';



function Formelbereich(props: any) {

    let formeln = [<Formel name="phi1" />,
    <Formel name="phi2" />,
    <Formel name="phi3" />];
    const [tabelle, setTabelle] = useState("");

    const generateTabelle = (event: HTMLButtonElement) => {
        let namen: String[] = [];
        formeln.forEach(element => {
            console.log(element.props.name);
            namen.push(element.props.name);
        });
        invoke("get_wahrheitstabelle_cmd", { namen }).then((value: any) => {
            setTabelle(value);
            console.log(value);

        }).catch((e) => {
            console.log(e);
        });
        console.log("tabelle");
    }

    const copy = (event: any) => {
        // // Get the text field
        // var copyText = document.getElementById("out-tabelle");

        // // Select the text field
        // copyText.select();
        // copyText.setSelectionRange(0, 99999); // For mobile devices

        // Copy the text inside the text field
        navigator.clipboard.writeText(tabelle);

        // Alert the copied text
        alert("Copied the text: " + tabelle);
    }

    return (

        <Stack className='formelbereich'>
            <Stack direction="row" >

                <Stack>
                    {formeln}
                </Stack>

                <Werkzeugkasten onTabelle={generateTabelle} />
            </Stack>
            {
                tabelle != "" &&
                <textarea id="out-tabelle" value={tabelle}></textarea>
            }
            {
                tabelle != "" &&
                <Button className='button-text' onClick={copy}>Copy</Button>
            }
        </Stack>
    );
}

export default Formelbereich;