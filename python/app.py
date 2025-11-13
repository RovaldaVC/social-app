from fastapi import FastAPI
from database import models, database
from routers import users, authentication

app = FastAPI()

# Create tables
models.Base.metadata.create_all(database.engine)

# Include routers
app.include_router(users.router)
app.include_router(authentication.router)