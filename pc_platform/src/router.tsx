import { Suspense } from 'react'
import { Routes, Route } from 'react-router-dom'
import Login from "@/pages/login";

const Router = () => {


    return (
        <Suspense fallback={<div>Loading...</div>}>
            <Routes>
                <Route path='/' element={<Login/>}/>
                {/*<Route path='/stage/*' element={<Layout/>}/>*/}
            </Routes>
        </Suspense>
    )
}

export default Router