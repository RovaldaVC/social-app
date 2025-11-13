from fastapi import Depends, HTTPException, status
from sqlalchemy.orm import Session
from database.database import get_db
from database import schema, models
from security.hashing import Hash
from jwt import Token

def auth(request:schema.Auth, db:Session=Depends[get_db]):
    user = db.query(models.usersDB).filter(models.usersDB.username == request.username).first()
    if not user:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Invalid credentials."
              )
    if not Hash.verify(request.password, user.password):
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Invalid credentials."
        )
    
    access_token = Token.create_access_token(data={"sub":user.username})
    return{
        "access_token":access_token,
        "token_type":"bearer"
    }