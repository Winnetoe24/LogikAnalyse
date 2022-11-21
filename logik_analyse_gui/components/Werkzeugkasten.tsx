import React, { ChangeEvent, useState } from 'react';
import Stack from '@mui/material/Stack';
import Button from '@mui/material/Button';

function Werkzeugkasten(props: any) {


    return(
    <Stack className='formel'>
        <Button className='button-text' onClick={props.onTabelle}>Tabelle</Button>
        <Button className='button-text'>Equivalenz</Button>
    </Stack>
    );
}

export default Werkzeugkasten;