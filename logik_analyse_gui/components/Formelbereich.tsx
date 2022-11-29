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

    const [formelnCount, setFormelnCount] = useState(1);
    const [selection, setSelection] = useState([false]);

    const [formeln, setFormeln]= useState([<Formel name="phi0" i={0} selection={[selection, setSelection]}/>]);

    
    const createNextFormel = () => {
        selection.push(false)
        setSelection(selection);
        let name = "phi"+formelnCount;
        let formel = <Formel name={name} i={formelnCount} selection={[selection, setSelection]}/>
        setFormeln(formeln.concat(formel));
        
        console.log(formeln);
        setFormelnCount(formelnCount+1);
        return formel;
    }
    
    const [tabelle, setTabelle] = useState("");

    const generateTabelle = (event: HTMLButtonElement) => {
        let namen: String[] = [];
        formeln.forEach(element => {
            if (element.props.selection[0][element.props.i]) {
                console.log(element.props.name);
                namen.push(element.props.name);
    
            }
        });
        if (namen.length == 0) {
            setTabelle("Bitte wähle mindestens eine Formel aus");
            return;
        }
        invoke("get_wahrheitstabelle_cmd", { namen }).then((value: any) => {
            setTabelle(value);
            console.log(value);
        }).catch((e) => {
            console.log(e);
        });
        console.log("tabelle");
    }

    const aequivalenz = (event: HTMLButtonElement) => {
        console.log("Äquivalenz");
        let namen: String[] = [];
        formeln.forEach(element => {
            if (element.props.selection[0][element.props.i]) {
                console.log(element.props.name);
                namen.push(element.props.name);
    
            }
        });
        if (namen.length < 2) {
            setTabelle("Bitte wähle mindestens zwei Formeln aus");
            return;
        }
        invoke("is_aequivalent", { namen }).then((value: any) => {
            setTabelle(value);
            console.log(value);
        }).catch((e) => {
            console.log(e);
        });
    }

    const copy = (event: any) => {
        // Copy the text inside the text field
        navigator.clipboard.writeText(tabelle);

    }

    const handleNewFormel = (event: any) => {
        let formel = createNextFormel();


    }

    const handleHelp = (event: any) => {
         setTabelle("Gebe deine Formeln in die Textfelder ein. \nZum einfacheren Eingeben gibt es ein Mapping zu Ascii Charakteren. \n t = top \n f = bottom \n & = and \n | = or \n\nUm neue Funktionen hinzuzufügen klicke auf das + \nUm eine Wahrheitstabelle auszugeben klicke auf Tabelle. \nUm zu prüfen um Formeln äquivalent sind, klicke auf Äquivalenz");
    }
    return (

        <Stack className='formelbereich'>
            <Stack direction="row" >

                <Stack>
                    {formeln}
                    <Button className='button-text' onClick={handleNewFormel}>+</Button>
                </Stack>

                <Werkzeugkasten onTabelle={generateTabelle} onAequivalenz={aequivalenz} onHelp={handleHelp} />
            </Stack>
            {
                tabelle != "" &&
                <textarea id="out-tabelle" value={tabelle} readOnly={true}></textarea>
            }
            {
                tabelle != "" &&
                <Button className='button-text' onClick={copy}>Copy</Button>
            }
        </Stack>
    );
}

export default Formelbereich;

function getNum() {
    throw new Error('Function not implemented.');
}
