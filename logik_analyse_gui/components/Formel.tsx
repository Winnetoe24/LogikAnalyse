import { Button, Checkbox, Grid, TextField } from '@mui/material';
import { ChangeEvent, FunctionComponent, useState } from 'react';

import { invoke } from '@tauri-apps/api/tauri';

const isClient = typeof window !== 'undefined'

type InputProps = { // The common Part
  className?: string;
  value?: string;
  placeholder?: string;
} & ({ // The discriminated union
  type?: "text";
  onChange?: (e: React.ChangeEvent<HTMLInputElement>) => void;
} | {
  type: "textarea";
  onChange?: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
})

const Input: FunctionComponent<InputProps> = (props: InputProps) => {
  if (props.type === 'textarea') {
      return <textarea {...props} />;
  }
  return <input {...props} />;
};


function Formel(props: any) {
  const [value, setValue] = useState("");
  const [isFormelOk, setFormelOk] = useState(false);
  const [isFormelWrong, setFormelWrong] = useState(false);
  const [isChecked, setChecked] = useState(false);


  const handleButton = () => {
    isClient &&
      invoke('renderFormel', { name: props.name, input: value })
        .then((formel: any) => {
          setFormelOk(true);
          setFormelWrong(false);
          console.log(formel);
          setValue(formel);
        }
        )
        .catch((formel: any) => {
          setFormelOk(false);
          setFormelWrong(true);
          console.error(formel);
        })
  }

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setFormelOk(false);
    setFormelWrong(false);
    console.log(event.target.value);
    setValue(event.target.value);
  }

  const handleChecked = (event: ChangeEvent) => {
    setChecked(!isChecked);
  }

  return (
    <Grid className='formel'>
      <Checkbox className='checkbox' onChange={handleChecked} />
      <div className='formelbereich1'>
        <h5>Formel "{props.name}"</h5>
        <Grid>
          <Input className='formelEingabe'
            value={value}
            onChange={handleChange}
          />
          <Button className='renderFormel' onClick={handleButton}>getFormel</Button>
        </Grid>
        {
          isFormelOk &&
          <p>OK</p>
        }
        {
          isFormelWrong &&
          <p>Fehler</p>
        }
      </div>

    </Grid>);
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

