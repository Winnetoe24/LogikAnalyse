import { Button, Checkbox, FormControlLabel, Grid, Stack, Switch, TextField } from '@mui/material';
import { ChangeEvent, Dispatch, FunctionComponent, SetStateAction, useState } from 'react';

import { invoke } from '@tauri-apps/api/tauri';

const isClient = typeof window !== 'undefined'

type InputProps = { // The common Part
    className?: string;
    value?: string;
    placeholder?: string;
} & ({ // The discriminated union
    type?: "text";
    onChange?: (e: React.ChangeEvent<HTMLInputElement>) => void;
    onBlur?: (e: React.FocusEvent<HTMLInputElement>) => void;
} | {
    type: "textarea";
    onChange?: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
    onBlur?: (e: React.FocusEvent<HTMLTextAreaElement>) => void;
})

const Input: FunctionComponent<InputProps> = (props: InputProps) => {
    if (props.type === 'textarea') {
        return <textarea {...props} />;
    }
    return <input {...props} />;
};


export interface FormelProps {
    name: string;
     i: number;
    selection: [boolean[], Dispatch<SetStateAction<boolean[]>>];
}


function Formel(props: FormelProps): JSX.Element{
    const [eingabe, setEingabe] = useState("");
    const [isFormelOk, setFormelOk] = useState(false);
    const [isFormelWrong, setFormelWrong] = useState(false);
    const [isUTF, setUTF] = useState(false);


    const renderFormel = () => {
        isClient &&
            invoke('renderFormel', { name: props.name, input: eingabe })
                .then((formel: any) => {
                    setFormelOk(true);
                    setFormelWrong(false);
                    console.log(formel);
                    getFormel();
                }
                )
                .catch((formel: any) => {
                    setFormelOk(false);
                    setFormelWrong(true);
                    console.error(formel);
                });
    }

    const checkFormel = (neueEingabe: string) => {
        console.log("check start"+eingabe); 
        isClient &&
            invoke('check_formel', { name: props.name, input: neueEingabe })
                .then((formel: any) => {
                    console.log("check ok");
                    setFormelOk(true);
                    setFormelWrong(false);
                })
                .catch((formel: any) => {
                    console.error("error check:" + formel);
                    setFormelOk(false);
                    setFormelWrong(true);
                })
        if (!isFormelOk) {
            setFormelWrong(true);
        }
    }

    const handleFocus = (event: React.FocusEvent<HTMLInputElement>) => {
        renderFormel();
    }
    
    const getFormel = () => {
        getFormelBool(isUTF);
    }

    const getFormelBool = (is_utf: boolean) => {
        isClient &&
            isFormelOk &&
            invoke('getFormel', { name: props.name, is_utf: is_utf })
                .then((formel: any) => {
                    console.log("formel:" + formel + " utf:" + isUTF);
                    setEingabe(formel);
                })
                .catch((formel: any) => {
                    console.error("getFormel");
                    console.error(formel);
                })
    }
    const handleUTF = (event: any) => {
        console.log("handleUTF selection:"+props.selection[0]);
        if (!isFormelOk) {
            renderFormel();
        }
        getFormelBool(!isUTF);
        setUTF(!isUTF);
        console.log("handleUTF selection:"+props.selection[0]);
    }

    const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        console.log("change");
        setFormelOk(false);
        setFormelWrong(false);
        console.log("change value:" + event.target.value);
        setEingabe(event.target.value);
        checkFormel(event.target.value);
    }

    const handleChecked = (event: ChangeEvent) => {
        let ar = props.selection[0];
        ar[props.i] = !ar[props.i]; 
       props.selection[1](ar);
    }


    return (

        <Grid className='formel'>
            <Checkbox className='checkbox' onChange={handleChecked} />
            <div className='formelbereich1'>
                <h5>Formel "{props.name}"</h5>
                <Stack direction="row">
                    <Input className='formelEingabe'
                        type="text"
                        value={eingabe}
                        onChange={handleChange}
                        onBlur={handleFocus}
                    />
                    <div className="utf-switch">
                        <FormControlLabel label={(isUTF ? "???UTF???" : "ASCII")}
                            control={<Switch checked={isUTF} onClick={handleUTF}></Switch>} />
                    </div>
                </Stack>
                {
                    isFormelOk &&
                    <p>OK</p>
                }
                {
                    isFormelWrong &&
                    <p>Fehler</p>
                }
            </div>
           
        </Grid>
);
}

// class Formel extends React.Component {
//   constructor(props: {} | Readonly<{}>) {
//     super(props);
//     const [value, setValue] = useState("");
//     this.handleChange = this.handleChange.bind(this); 
//     this.handleButton = this.handleButton.bind(this); 
//   }


//   render(): React.ReactNode {
//     return (
//       <div className='formel'>
//         <h5>Formel "{this.props.name}"</h5>
//           <textarea className='formelEingabe' value={this.state.value} onChange={this.handleChange}></textarea>
//           <button className='renderFormel' onClick={this.handleButton}>getFormel</button>
//       </div>);
//   }
// }
export default Formel;

