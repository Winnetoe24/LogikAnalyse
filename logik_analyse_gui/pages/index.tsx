import Head from 'next/head'
import Image from 'next/image'
import styles from '../styles/Home.module.css'
import { NextPage } from 'next'
import { useEffect } from 'react'
import Formel from '../components/Formel'
import Paper from '@mui/material/Paper';
import Stack from '@mui/material/Stack';
import Button from '@mui/material/Button';
import { styled } from '@mui/material/styles';
import Werkzeugkasten from '../components/Werkzeugkasten'
import Formelbereich from '../components/Formelbereich'



const Home: NextPage = () => /* useEffect(() => {*/ /*   invoke('greet', { name: 'World' })*/ /*     .then(console.log)*/ /*     .catch(console.error)*/ /* }, []);*/(
  <div>

    <h1>Aussagenlogik</h1>
    <Formelbereich />

  </div>
)

export default Home 