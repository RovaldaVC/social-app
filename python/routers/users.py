from fastapi import APIRouter, status, Depends
from database.database import get_db
from sqlalchemy.orm import Session
from database import schema
from repository import users

router = APIRouter(
    prefix="/user",
    tags=["users"]
)

@router.post("/", status_code=status.HTTP_201_CREATED, response_model=schema.ShowUser,)
def create(request:schema.CreateUser, db:Session=Depends(get_db)):
    createFunction = users.create_user(request, db)
    return createFunction   