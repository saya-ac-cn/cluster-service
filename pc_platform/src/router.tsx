import { Suspense } from 'react'
import { Routes, Route } from 'react-router-dom'
import Login from "@/pages/login";
import Layout from "@/pages/layout/backend-v1";

const Router = () => {


    return (
        <Suspense fallback={<div>Loading...</div>}>
            <Routes>
                <Route path='/' element={<Login/>}/>
                <Route path='/backstage/*' element={<Layout/>}/>
            </Routes>
        </Suspense>
    )
}

export default Router